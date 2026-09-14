# Vocabulary Specification

## Purpose

Rules governing names exposed by the method, record, command line, and messages. The writing style governs sentences. This domain governs the terms those sentences and interfaces use.

## Requirements

### `vocabulary:a-public-name-states-one-concept` — A public name states one concept

The author MUST give each public domain, concept, field, and field value one plain-language name that states the concept without assumed knowledge of another method.

#### Scenario: A field uses inherited process terminology

- GIVEN an entry field whose name identifies a category but not the fact it records
- WHEN a reader meets the field without prior knowledge of its source method
- THEN the name states the recorded fact and the reader does not need an external vocabulary

Verify: reviewer reads each introduced or changed public name with its local example and confirms that it states one concept without external terminology

### `vocabulary:a-public-name-is-lean` — A public name is lean

Where two names state the same concept, the author MUST use the shorter name that preserves the distinction from adjacent concepts.

#### Scenario: A name carries a redundant category word

- GIVEN a field name whose shorter form retains its complete meaning
- WHEN the public vocabulary is reviewed
- THEN the shorter form remains and the redundant word is removed

Verify: reviewer confirms that each introduced or changed public name contains no word whose removal preserves its complete meaning

### `vocabulary:the-owning-page-defines-a-public-term` — The owning page defines a public term

When a public term first appears on its owning page, the author MUST define its meaning, its allowed values where finite, and its boundary with adjacent concepts.

#### Scenario: A configuration key controls one part of a procedure

- GIVEN a reader who knows the key name but not the procedure
- WHEN the reader opens the owning specification
- THEN the definition says what the key controls and what remains fixed

Verify: reviewer confirms that each introduced or changed public term is defined at first use with its values and boundary

### `vocabulary:one-concept-keeps-one-public-name` — One concept keeps one public name

The author MUST use one public name for one concept across specifications, schemas, examples, guides, and messages.

#### Scenario: A schema and a guide name one field differently

- GIVEN two surfaces that refer to the same record fact
- WHEN a reader follows the field from the guide into the schema
- THEN both surfaces use the same name

Verify: reviewer searches for synonyms of each introduced or changed public term and confirms that every current surface uses its canonical name

## Unenforced rules

| Rule                                               | Reviewer confirms                                                                              |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `vocabulary:a-public-name-states-one-concept`      | A local example makes the concept clear without external terminology                           |
| `vocabulary:a-public-name-is-lean`                 | Removing another word would remove meaning or a needed distinction                             |
| `vocabulary:the-owning-page-defines-a-public-term` | The first definition states meaning, finite values, and the boundary                           |
| `vocabulary:one-concept-keeps-one-public-name`     | Specifications, schemas, examples, guides, and messages use the same name for the same concept |
