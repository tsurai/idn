+++
title = "Basic sentence structure & context"
slug = "01-basic-sentence-structure"
date = "2025-09-13"
updated = "2025-10-24"
[extra]
next = "02-pronouns.md"
summary = "The first introduction to _Indonesian_. Learn about _SVO_, the basic sentence structure, how to construct basic statements, as well as some interesting quirks that make _Indonesian_ special."
+++

> From this point on _Indonesian_ specifically refers to _Bahasa Indonesia_. While a lot of the presented information is shared with _Bahasa Malayu_, both being offspring of _Classical Malay_, I'm neither educated enough, nor have the cultural understanding to point out the differences and nuances. Please feel free to contact me when you find mixups and errors.

Similar to _English_ and most other Indo-European languages _Indonesian_ generally uses

> **Subject - Verb - Object (SVO)**

While this is the most commonly used form, _Indonesian_ offers a lot of flexibility depending on context and desired emphasis. Made especially easy due to the general lack of conjugations, tenses, and gender nouns which will be discussed in detail later. Words are often omitted in casual conversation when the context is enough to infer the intended meaning. The specifics will become clear with enough practice.

<h5>Vocabulary</h5>
<dl class="card grid4">
{{ vocab(id="saya", en="I", details="formal", pos="pronoun") }}
{{ vocab(id="suka", en="to like", pos="verb root") }}
{{ vocab(id="makan", en="to eat", pos="verb root") }}
{{ vocab(id="menulis", en="to write", pos="verb") }}
{{ vocab(id="kucing", en="cat", pos="noun") }}
{{ vocab(id="apel", en="apple", pos="noun") }}
{{ vocab(id="surat", en="letter", pos="noun") }}
</dl>

<h5>Examples</h5>
<dl class="card examples">
{{ vocab(id="Saya suka kucing", en="I like cats", pos="personal pronoun + verb + noun", verbose=[
  [["Saya", "perp"], "S"], ["suka", "v"], [["kucing", "n"], "O"]
]) }}
{{ vocab(id="Saya makan apel", en="I'm eating an apple", pos="personal pronoun + verb + noun", verbose=[
  [["Saya", "perp"], "S"], ["makan", "v"], [["apel", "n"], "O"]
]) }}
{{ vocab(id="Saya menulis surat", en="I'm writing a letter", pos="personal pronoun + verb + noun", verbose=[
  [["Saya", "perp"], "S"], ["menulis", "v"], [["surat", "n"], "O"]
]) }}
</dl>

The first two simple sentences showcase how context heavy _Indonesian_ is, especially in casual daily conversations that use a lot of abbreviations. The first sentence, {{id(id="Saya suka kucing")}}, despite meaning **I like cats**, literally translates to "**I like cat**", with **cat** as a singular. The proper way to form the plural of a noun is by repetition, hence {{id(id="kucing-kucing", en="cats")}} is **cats**.

So why is {{id(id="kucing", en="cat")}} used instead? Because there is nothing that would indicate a specific singular cat, like for example _this_ or _that_ cat. This leads to the assumption that the speaker is talking about cats in general despite using {{id(id="kucing", en="cat")}} instead of {{id(id="kucing-kucing", en="cats")}}.

The second sentence {{id(id="Saya makan apel", en="I'm eating an apple")}} is similar but with regard to the verb tense instead. On its own and without further context it would most likely be translated to "**I'm eating an apple**". But if it was the answer to someone asking "**What kind of fruits do you eat?**", suddenly its translation would change to "**I eat apples**" instead. Notice how in this case both the tense of **eat** and the number of **apple** changes.

There are words that can be used to be more specific if needed, {{id(id="Saya sedang makan apel", en="I'm eating an apple")}}, for example, explicitly states that you are **eating** right now.

_Indonesian_ has its own formal grammar and rules just like any other language, but is a lot more flexible in its usage. The more you drift into casualness and slang the further you deviate from the textbooks. Especially in text you might find things like {{id(id="kucing2", en=" cats")}} instead of {{id(id="kucing-kucing", en="cats")}} to say **cats**. This might sound a little daunting at first, but it also makes constructing sentences very easy and requires less words and concepts to be memorized to express yourself.
