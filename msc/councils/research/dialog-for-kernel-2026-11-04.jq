def stamp: .timestamp[0:16] | sub("T";" ") + "Z";
def peer: test("49825|Major refactor effort status");
select(.timestamp >= $from and .timestamp < $to)
| if .type=="attachment" and .attachment.type=="queued_command" then
    ((.attachment.prompt // .attachment.content // "") | if type=="string" then . else tostring end) as $p
    | if ($p|startswith("<cross-session-message")) then (if ($p|peer) then "\n#### vsect lead (\"Major refactor effort status\") → councils-46 — " + stamp + "\n\n" + $p else empty end)
      elif ($p|startswith("<")) then empty
      else "\n#### Joseph — " + stamp + " (typed mid-turn)\n\n" + $p end
  elif .type=="user" and (.message.content|type)=="string" then
    .message.content as $c
    | if ($c|startswith("Another Claude session sent a message")) then (if ($c|peer) then "\n#### vsect lead (\"Major refactor effort status\") → councils-46 — " + stamp + "\n\n" + $c else empty end)
      elif ($c|startswith("<")) or ($c|startswith("This session is being continued")) then empty
      else "\n#### Joseph — " + stamp + "\n\n" + $c end
  elif .type=="assistant" then
    stamp as $t | .message.content[]
    | if .type=="text" then "\n#### " + $who + " — " + $t + "\n\n" + .text
      elif .type=="tool_use" and .name=="SendMessage" and (.input.to|peer) then "\n#### " + $who + " → message to the vsect lead — " + $t + "\n\n" + (.input.message)
      else empty end
  else empty end
