## _variables.scss
: 38
```diff
  // scss-docs-start color-variables
- $blue:    #0d6efd !default;
+ $blue:    #07689F !default;
  $indigo:  #6610f2 !default;
  $purple:  #6f42c1 !default;
- $pink:    #d63384 !default;
+ $pink:    #e83e8c !default;
- $red:     #dc3545 !default;
+ $red:     #F67280 !default;
  $orange:  #fd7e14 !default;
  $yellow:  #ffc107 !default;
- $green:   #198754 !default;
+ $green:   #11d3bc !default;
  $teal:    #20c997 !default;
- $cyan:    #0dcaf0 !default;
+ $cyan:    #a2d5f2 !default;
```
:300
```diff
  // scss-docs-start theme-color-variables
  $primary:       $blue !default;
- $secondary:     $gray-600 !default;
+ $secondary:     $gray-400 !default;
  $success:       $green !default;
```
:451
```diff
  // Links
  //
  // Style anchor elements.

  $link-color:                              $primary !default;
- $link-decoration:                         underline !default;
+ $link-decoration:                         none !default;
  $link-shade-percentage:                   20% !default;
  $link-hover-color:                        shift-color($link-color, $link-shade-percentage) !default;
- $link-hover-decoration:                   null !default;
+ $link-hover-decoration:                   underline !default;
```
:1520
```diff
- $modal-backdrop-opacity:            .5 !default;
+ $modal-backdrop-opacity:            .7 !default;
```

## /mixins/_table-variants.scss
```diff
-    --#{$prefix}table-color: #{$color};
+    --#{$prefix}table-color: var(--#{$prefix}-emphasis-color);
-    --#{$prefix}table-bg: #{$background};
+    --#{$prefix}table-bg: var(--#{$prefix}#{$state}-bg-subtle);
-    --#{$prefix}table-border-color: #{$table-border-color};
-    --#{$prefix}table-striped-bg: #{$striped-bg};
+    --#{$prefix}table-striped-bg: var(--#{$prefix}#{$state}-bg-subtle);
-    --#{$prefix}table-striped-color: #{color-contrast($striped-bg)};
+    --#{$prefix}table-striped-color: var(--#{$prefix}-emphasis-color);
     --#{$prefix}table-active-bg: #{$active-bg};
     --#{$prefix}table-active-color: #{color-contrast($active-bg)};
-    --#{$prefix}table-hover-bg: #{$hover-bg};
+    --#{$prefix}table-hover-bg: var(--#{$prefix}#{$state}-bg-subtle);
-    --#{$prefix}table-hover-color: #{color-contrast($hover-bg)};
+    --#{$prefix}table-hover-color: var(--#{$prefix}#{$state}-text-emphasis);
```

# Remove javascript
- tab: using class_signal("active"), remove "tab-pane" in "tab-content"
- pill: using class_signal("active"), remove "tab-pane" in "tab-content"
- collapse: accordion|navbar toggler|announcement use mutable(enum/bool) using
    1. prop_signal("aria-expanded") on "navbar-toggler" element
    2. swap mutable value by click event on "navbar-toggler" element
    3. class_signal("show") on "navbar-collapse"+"collapse" element
- dropdown: NOTE "dropdown-menu-end" need "data-bs-popper" attribute for css
    1. class_signal("show") on "dropdown-toggler" element
    2. prop_signal("aria-expanded") on "dropdown-toggler" element
    3. swap mutable value by click event on "dropdown-toggler" element
    4. class_signal("show") on "dropdown-menu" element
    5. global click + "Esc" keydown event on "dropdown" or container element
- radio button: using class_signal("active")
- modal
    1. show "modal-backdrop" on Some or true mutable, hide "modal-backdrop" on None or false mutable
    2. child_signal(mutable) to render modal
    3. global click + "Esc" keydown event on "modal" element