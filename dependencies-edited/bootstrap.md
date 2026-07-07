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