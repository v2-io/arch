"""Tiny JSON-RPC stdio client for smoke-testing the concilium server (no model)."""
import json, subprocess, sys, threading, queue, time
class Client:
    def __init__(self, argv, env=None):
        self.p = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1, env=env)
        self.q = queue.Queue(); self.id = 0
        threading.Thread(target=self._rd, daemon=True).start()
    def _rd(self):
        for line in self.p.stdout: self.q.put(json.loads(line))
    def req(self, method, params=None, timeout=60):
        self.id += 1; rid = self.id
        self.p.stdin.write(json.dumps({"jsonrpc":"2.0","id":rid,"method":method,"params":params or {}})+"\n"); self.p.stdin.flush()
        notes=[]; t0=time.time()
        while time.time()-t0<timeout:
            m=self.q.get(timeout=timeout)
            if m.get("id")==rid: return m, notes
            notes.append(m)
        raise TimeoutError
    def call(self, name, args, meta=None, timeout=60):
        m,n=self.req("tools/call",{"name":name,"arguments":args,"_meta":meta or {"progressToken":self.id+1}},timeout)
        return m["result"]["content"][0]["text"], m["result"].get("isError"), n
    def close(self): self.p.stdin.close(); self.p.wait(5)
