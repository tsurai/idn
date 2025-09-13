+++
title = "Conjugation: Tense & Voice"
slug = "04-verb-modification"
date = "2025-09-13"
updated = "2025-09-13"
[extra]
prev = "03-greetings-and-farewells.md"
next = "05-asking-questions.md"
summary = "Strictly speaking, _Indonesian_ does not have verb conjugation and uses modifications made with various affixes and adverbs instead. This chapter introduces these concepts and how to modify the tense of a verb or clause."
+++

Strictly speaking, the title of this chapter is a lie. Unlike many languages, especially western languages, _Indonesian_ does not actually use conjugations. Instead of using different verbs depending on the tense, i.e. **eat** turning into **ate** or **eaten**, _Indonesian_ relies on _adverbs/particles_ and _affixes_ to supplement a verb with additional meaning.

#### Tense / Aspect
Instead of _tense_ to describe the point in time of an action _Indonesian_ utilizes the grammatical concept of _aspect_ to express the extension of a state or action in time. At a glance these two seem to be the same which is very confusing.

##### Vocabulary
<dl class="card grid2">
{{ vocab(id="sedang", en="currently / in process", pos="particle", detail="progressive aspect") }}
{{ vocab(id="sudah", en="already / has / have", pos="particle", details="casual past aspect") }}
{{ vocab(id="telah", en="already / has / have", pos="particle", details="formal past aspect") }}
{{ vocab(id="akan", en="will / going to", pos="particle", details="future") }}
{{ vocab(id="belum", en="not yet", pos="particle", details="negative perfect") }}
{{ vocab(id="masih", en="still", pos="particle", details="continuous") }}
</dl>

To illustrate the difference, let's have a look at {{id(id="sudah", en="already / has / have")}}, the aspect for a _completed_ action. The English phrases **I ate lunch** _(simple past)_ and **I have eaten lunch** _(present perfect)_ are distinct tenses but use the same _aspect_ and thus, would both be expressed with {{id(id="sudah", en="already / has / have")}}. As always there are nuances and in _Indonesian_ context is king. In {{id(id="Aku makan nasi kemarin", en="I ate rice yesterday")}} which translates to **I ate rice yesterday**, {{id(id="sudah", en="already / have / has")}} is not needed because it is implied by {{id(id="kemarin",en="yesterday")}} / **yesterday**.

##### Examples
<dl class="card">
{{ vocab(id="Aku sudah makan nasi", en="I ate rice", pos="subject + particle + verb + object") }}
{{ vocab(id="Aku akan makan nasi", en="I will eat rice", pos="subject + particle + verb + object") }}
{{ vocab(id="Besok aku akan sudah makan nasi", en="I will have eaten rice tomorrow", pos="noun + subject + particle + particle + verb + object") }}
{{ vocab(id="Aku belum belajar", en="I haven't studied yet", pos="subject + particle + verb") }}
{{ vocab(id="Aku masih belajar indonesia", en="I'm still learning Indonesian", pos="subject + particle + verb + object") }}
</dl>

##### Quiz
{{quiz(id="Saya _ minum air", en="I have drunk water", options=["sudah","akan","sedang","telah"], answers=["sudah","telah"], note="Sudah and telah both indicate the correct aspect but since the subject is Saya, which is formal, it would sound more natural to use telah")}}

#### Voice
Another way to modify verbs is the usage of _affixes_ (_prefixes_, _suffixes_ and _circumfixes_) to indicate _voice_ or _moods_. As easy and straightforward as other parts of _Indoneesian_ grammar are, how to apply the affixes and their effects are complex and full of exceptions. Given the correct _affix_ a verb can be made active, passive, habitual, accidental, and a lot more. For now we will only focus on a few of them that are frequently used.

The _base_ or _root form_ of a word, {{id(id="kata dasar", en="basic word")}} in _Indonesian_, falls into one of two categories: **Verb Roots** and **Non-Verb Root**.

The former are already usable verbs, but the later _Non-Verb Roots_ _have_ to be affixed to be used as a verb. Previous lessons already used words of both categories: {{id(id="makan", en="to eat")}} / **to eat** is a _Verb Root_ while {{id(id="menulis", en="to write")}} is the affixed version of the _Non-Verb Root_ {{id(id="tulis", en="writing")}}. Nowadays the _prefix_ to turn _Non-Verb Roots_ into real _verbs_ is often dropped in casual conversations. While this is largely accepted and natural, it is not grammatically correct.

##### Agent focus me-
The _agent focus_ **me-** turns the _root word_ into an _active transitive verb_, a verb with one or more _objects_. For example, {{id(id="baca", en="read")}} / **read** becomes {{id(id="membaca", en="to read")}} / **to read** according to the construction rules noted below.

You might think that **me-** merely adds a **to ___** to a given root word but this is incorrect. Instead, it's more like applying the concept or essence of the root to something, which can lead to surprising results. Consider {{id(id="Hukum", en="law")}} the word for **law**. It wouldn't make a lot of sense to say **to law** so what would _doing the law to something/someone_ be? In this case {{id(id="menghukum", en="to punish")}} means **to punish**.

<table>
  <tr>
    <th>Prefix</th>
    <th>Initial Consonant</th>
  </tr>
  <tr>
    <td>meng-</td>
    <td>(vowel), g, h, k</td>
  </tr>
  <tr>
    <td>mem-</td>
    <td>b, p, f</td>
  </tr>
  <tr>
    <td>men-</td>
    <td>c, d, j, sy, t, z</td>
  </tr>
  <tr>
    <td>me-</td>
    <td>l, m, n, ny, ng, r, w, y</td>
  </tr>
  <tr>
    <td>meny-</td>
    <td>s</td>
  </tr>
  <tr>
    <td>menge-</td>
    <td><em>all else</em></td>
  </tr>
</table>

##### Examples
<dl class="card">
{{ vocab(id="Dia menulis surat", en="She writes a letter", pos="subject + verb (me- + tulis) + object") }}
{{ vocab(id="Aku membaca buku", en="I read a book", pos="Subject + verb (me- + baca) + object") }}
{{ vocab(id="Saya makan apel", en="I eat an apple", pos="Subject + verb + object", details="makan is Verb Root word and doesn't need me-") }}
</dl>

##### Patient focus
The functional role of the _passive_ in _English_ and _Indonesian_ differ significantly. In _English_ the _passive_ is used less frequently, often employed for rhetorical effect. But in _Indonesian_ the _passive_ is a lot more common and natural in every day conversations. Furthermore, it is used in the context of politeness, like conveying requests or commands while maintaining social norms.

From a linguistic point of view there are a lot of other, deeply rooted differences between _English_ and _Indonesian_. One uses an _asymmetrical voice_ system, the other, being a part of the _Austronesian_ language family, a _symmetrical voice_ system. That's why, for example, linguists prefer the term _patient focus_ over _passive voice_ for _Indonesian_. That being said it is still an ongoing debate and even in _Indonesian_ textbooks you will see the terms _active_ and _passive_ due to western colonization infuence. That being said, in my opinion it doesn't really matter which term you use as an beginner or intermediate learner, the knowledge that these different terms exist is enough for the moment.

<details>
  <summary>Nerd details</summary>

  In _English_ the _active_ is the default and the _passive_ is _derived_ from it. Grammatically the _English active_ has a higher status and is used way more frequently; their usage is _asymmetrical_. When constructing the _passive_ in _English_ the _patient_ or _undergoer_ gets _promoted_ to the _subject_ and the _actor_ gets _demoted_ to an _optional prepositional_. In stark contrast, the _Indonesian passive_ isn't just a derived form, but a mutual to the _active_. It's construction doesn't _promote_ or _demote_, the _patient_ and _actor_ are both equal in status; they are in _symmetrical_ relationship.

</details>

##### Type 1 di- passive
The _Type 1 patient focus_ **di-** turns the _root word_ into a _passive verb_ and shifts the focus from the actor to the undergoer. Given the _active sentence_ {{id(id="Michi makan apel", en="Michi eats an apple")}} / **Michi eats an apple**, the _passive_ {{id(id="Apel itu dimakan Michi", en="The apple was eaten by Michi")}} / **The apple was eaten by Michi**, transforms the _Subject_ of the _active_ into the _Object_ of the _passive_ sentence and vice versa.

Unlike the _active focus_ **me-**, there are no complicated construction rules for the _passive focus_ **di-**, instead it is simply added to the _root word_ without any further modifications.

1. _Undergoer (Subject) + di-Verb_
2. _Undergoer (Subject) + di-Verb + Actor (Object)_
3. _Undergoer (Subject) + di-Verb + ? + oleh + Actor (Object)_

Depending on the content, different forms can be used to construct a _passive_ sentence. The basic form consists of the _subject_ and the di- prefixed _verb_ followed directly by the _actor_, which when unknown or obvious by the context, can even be omitted as well.

But if the _verb_ and _actor_ are separated by another phrase, the word {{id(id="oleh",en="by")}} / **by** has to be added before the _actor_ to mark it as such. **By** in this context specifically only refers to the _actor_ and can be thought of as **because of**. Don't confuse it with **by means of** as in **by train**, because that is a completely different word, {{id(id="dengan",en="by means of / with")}}.

##### Type 2 short passive
This type of _passive_ is often used in casual conversations, but requires the _actor_ to be a 1st or 2nd person _pronoun_. Rather than modifying the _verb_ itself, the position of the _pronoun_ relative to the _verb_ changes to indicates the _passive_. It is placed either directly proceeding the _verb_ or added in its _possessive pronoun_ suffix form. The _type 2 passive_ cannot be combined with {{id(id="oleh", en="by")}} / **by** as the _actor_, in this case the _pronoun_ has to stay directly proceeding the _verb_.

1. _Undergoer (Subject) + 1st or 2nd Person Pronoun (Object) + Verb_
1. _Undergoer (Subject) + verb-ku,-mu,-nya_

##### Examples
<dl class="card">
{{ vocab(id="Tikus dikejar kucing", en="The rat is chased by a cat", pos="subject + verb + object") }}
{{ vocab(id="Dia aku pukul", en="He/She was hit by me", pos="subject + object + verb") }}
{{ vocab(id="Dia pukulku", en="He/She was hit by me", pos="subject + verb-ku") }}
{{ vocab(
  id="Buku yang aku pinjam udah aku balikin kemarin",
  en="The book I borrowed was already returned by me yesterday",
  pos="subject noun phrase + adverb + actor + verb + adverb",
  verbose=[
    [["Buku", "n"], ["yang", "conj"], ["aku", "p"], ["pinjam", "v"], "S"],
    ["uda", "adv"],
    [["aku", "p"], "A"],
    ["balikin", "v"],
    ["kemarin", "adv"]
  ])
}}
</dl>
