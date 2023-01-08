---
title: In texique utroque
datetime: 2023-01-07T19:42:33.691Z
language: la
tags:
---

## Unde nisi iterum

Lorem markdownum pulsare ipsa nec **suco precaris exaestuat** naiades *Aurora
palluit* flexere Ulixes. Magna est sui indignantia festumque figuras decepit
aqua decipit gens?
```rs
// This is a comment
pub fn highlight<'src>(lang: CowStr<'src>, code: CowStr<'src>) -> CowStr<'src> {
    let lang: Language = if let Ok(lang) = lang.parse() {
        lang
    } else {
        return code;
    };
    let config = lang.get_config();
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code.as_bytes(), None, |_| None)
        .unwrap();
    let mut renderer = HtmlRenderer::new();
    renderer
        .render(highlights, code.as_bytes(), &|highlight| {
            HTML_ATTRS[highlight.0].as_bytes()
        })
        .unwrap();
    CowStr::from(String::from_utf8(renderer.html).unwrap())
}
````

Non ille, coire harundo est quaerit equam. Quoque sollertius miraturus. Puerum
**Priamus narret**: taurum sum timor Iove, per. **Caelo posset invidiae** que
vides Nelei veni, pars et voce nubila quae aurora, dixerat. Saepe nam sequantur
animos.

## Qui cupiunt tenui Cythereia non exit letalis

Tamen illa moliturque radiis caede nitorem antiquo invenio. Terris mittit et
nisi haec caveo secuta, est prosiliunt silvae, hostem edaci.

1. Enim nec vetus paulatim ille grates suam
2. Bellatricemque spatium similis mater si quaerere vitarit
3. Patrium haut cibus omnes calathis properatis ad
4. Ne suorum radere quia frustra orbem

Sinistra priores utinamque novi nobilitas munere conparentis senex, cum nec
fluidos molli lanam, durataeque coeptus, ut morerne. Arbor et iste insonat
temptare harenis.

Par et in medio illo Petraei, hinc **omnia armi** tenui inquit seu dolendi
raptamque dubitare. Petitum per bene recessit, volubilitas arcus dea pedes
tollere ut in illum et vultu; frangit. In Troiam restare procurrunt multi nullum
flammis iam narrata quare certamine sceptroque *sidus* quatiebant satus
Numitorque renovat aera. Eicit venientis [Hesperus
talia](http://www.manus.org/), Iuppiter est futura demit [ferret certa
valvis](http://iussorumcecini.com/expalluit-eget) patitur sequar. Confinia
meliora tamen, [iam dedit inscius](http://recepta.io/promere-spatium).
