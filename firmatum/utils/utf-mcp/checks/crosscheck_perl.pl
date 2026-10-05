#!/usr/bin/env perl
# Cross-check script, block and age from `utf-mcp dump` against Perl's
# Unicode::UCD (an independent implementation of an older Unicode version;
# macOS system perl 5.34 carries Unicode 13.0). Only code points assigned by
# that version are compared.
#   perl tests/crosscheck_perl.pl /tmp/dump.tsv
use strict; use warnings;
use Unicode::UCD qw(charprop);
my $ver = Unicode::UCD::UnicodeVersion();
my @pv = split /\./, $ver;
my (%n, %ex); my $compared = 0;
open my $f, '<:encoding(UTF-8)', $ARGV[0] or die $!;
<$f>;
while (<$f>) {
  chomp; my @c = split /\t/, $_, -1;
  my ($cp, $age, $script, $block) = (hex $c[0], $c[10], $c[11], $c[12]);
  next if $age eq 'Unassigned';
  my @a = split /\./, $age;
  next if $a[0] > $pv[0] || ($a[0] == $pv[0] && $a[1] > $pv[1]);
  $compared++;
  my %chk = (
    script => [$script, charprop($cp, 'Script')],
    block  => [$block,  charprop($cp, 'Block')],
    age    => [$age,    charprop($cp, 'Age') =~ s/^V(\d+)_(\d+)$/$1.$2/r],
  );
  for my $k (keys %chk) {
    my ($x, $y) = @{$chk{$k}};
    (my $xn = lc $x) =~ s/[ _-]//g; (my $yn = lc($y // '')) =~ s/[ _-]//g;
    next if $xn eq $yn;
    $n{$k}++; push @{$ex{$k}}, sprintf("U+%04X %s vs %s", $cp, $x, $y // 'undef') if ($n{$k} <= 10);
  }
}
print "perl Unicode::UCD $ver: compared $compared code points\n";
print %n ? '' : "no mismatches\n";
for my $k (sort keys %n) { print "\n$k: $n{$k} mismatches (utf-mcp vs perl)\n"; print "  $_\n" for @{$ex{$k}}; }
