# Token / glyph comprehension survey

Qualitative. Each model was shown the character in a prompt (not an image).
The code point was also given; the prompt asked the model not to guess from
the number if the glyph was unreadable. That instruction is imperfect — treat
answers that recap the U+ number without describing a mark as weak evidence.

Models this run: llama3.2:3b

## U+0031 `1` DIGIT ONE

*why sampled:* ascii digit 1 — baseline

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: None
3. NONE
4. 0
```

## U+2160 `Ⅰ` ROMAN NUMERAL ONE

*why sampled:* roman numeral 1

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: (not readable)
3. Value: NONE
4. Confidence: 0
```

## U+2460 `①` CIRCLED DIGIT ONE

*why sampled:* circled digit 1

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: Not identifiable (code point U+2460 doesn't match any c[1D[K
common Unicode character)
3. Value: NONE
4. Confidence: 0
```

## U+3021 `〡` HANGZHOU NUMERAL ONE

*why sampled:* suzhou / hangzhou 1

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: Japanese kanji character for "one"
3. Value: 1
4. Confidence: 0 (the code point matches, but I couldn't identify a clear g[1D[K
glyph)
```

## U+4E00 `一` CJK UNIFIED IDEOGRAPH-4E00

*why sampled:* CJK 1 (ideograph)

### llama3.2:3b

```
1. Glyph: ( unreadable / REPLACEMENT )
2. Name or meaning: Unit
3. Value: NONE
4. Confidence: 0
```

## U+05D0 `א` HEBREW LETTER ALEF

*why sampled:* hebrew alef — gematria 1, UCD-silent

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: Hebrew letter Aleph (first letter of the Hebrew alphabe[7D[K
alphabet)
3. Value: U+05D0
4. Confidence: 0
```

## U+2588 `█` FULL BLOCK

*why sampled:* full block

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT 
2. Name or meaning: Right-to-left mark
3. Value: NONE
4. Confidence 0
```

## U+2591 `░` LIGHT SHADE

*why sampled:* light shade

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT
2. Name or meaning: Left-to-right Mark
3. Value (if a number): NONE
4. Confidence: 0
```

## U+2800 `⠀` BRAILLE PATTERN BLANK

*why sampled:* braille blank

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT / MOJIBAKE
2. Name or meaning: None
3. Value (if a number): NONE
4. Confidence 0-1: 0
```

## U+28FF `⣿` BRAILLE PATTERN DOTS-12345678

*why sampled:* braille all-8-dots

### llama3.2:3b

```
1. Glyph: UNREADABLE / REPLACEMENT / MOJIBAKE
2. Name or meaning: (UNREADABLE)
3. Value (if a number): NONE
4. Confidence: 0
```

