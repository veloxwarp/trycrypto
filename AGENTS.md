# TryCrypto contributor instructions

These instructions apply to the entire repository.

## Family brand system

The canonical shared brand guide lives in `snoyberg/snoyman.com`:

[Snoyman web family brand guidelines](https://github.com/snoyberg/snoyman.com/blob/master/docs/brand-guidelines.md)

Before substantial UI, styling, branding, or visual-content work, check the current canonical guide when GitHub access is available. Check it again before finalizing a substantial UI PR if the work may have introduced a new recurring visual pattern.

Stable family defaults include:

- light, calm surfaces with dark navy/ink text and restrained teal accents;
- Inter/system sans for ordinary UI;
- generous whitespace, readable measures, subtle borders, modest radii, and low-opacity shadows;
- clear keyboard focus, accessible contrast, responsive layouts, and reduced-motion support;
- product identity through a small number of deliberate deviations rather than unrelated palettes or component styles.

TryCrypto should remain an approachable educational expression of the family: off-white background, white lesson surfaces, dark code/output areas, generous spacing, and minimal visual noise.

Do not create a new local family-wide design rule. If work here reveals a change that should apply across the family, open or propose a companion PR against `snoyberg/snoyman.com` updating `docs/brand-guidelines.md`, then apply the change locally. Product-specific exceptions may remain local, but should be explicit and intentional.
