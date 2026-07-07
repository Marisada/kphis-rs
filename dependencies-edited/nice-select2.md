## nice-select2.js
:254
```diff
    const html = `
      <div class="${classes.join(" ")}" tabindex="${this.disabled ? "" : 0}">
        <span class="${this.multiple ? "multiple-options" : "current"}"></span>
        <div class="nice-select-dropdown">
          ${searchHtml}
          <ul class="list"></ul>
        </div>
      </div>
    `;

+   if (this.el.nextElementSibling) {
+     this.el.parentNode.removeChild(this.el.nextElementSibling);
+   }
    this.el.insertAdjacentHTML("afterend", html);

    this.dropdown = this.el.nextElementSibling;

    this.#renderSelectedItems();

    this.#renderItems();
```

:156 add
```js
  renderValue(value='') {
    const options = Array.from(this.el.querySelectorAll("option,optgroup"));
    if (this.multiple) {
      let values = value.split(',');
      if (values.length) {
        options.forEach(item => {
          if (values.includes(item.value)) {
            item.setAttribute("selected", true);
          }
        })
        this.update();
      }
    } else {
      options.forEach(item => {
        if (item.value == value) {
          this.dropdown.querySelector(".current").innerHTML = item.innerHTML;
          this.el.value = value;
        }
      })
    }
  }
```

:291
```diff
  #renderSelectedItems() {
    if (this.multiple) {
      let selectedHtml = "";

      if (
+       this.config.showSelectedItems ||
        window.getComputedStyle(this.dropdown).width === "auto" ||
        this.selectedOptions.length < 2
      ) {
-       this.selectedOptions.forEach( (item, index, array) => {
-         let text  = item.data.text;

-          if(index !== array.length -1 ){
-           text += `, `;
-         }

-         let span  = document.createElement("span");
-         span.classList.add("current");
-         span.textContent = text;

-         selectedHtml += span.outerHTML;
-       });
+       this.selectedOptions.forEach(function(item) {
+         selectedHtml += `<span class="current" data-value="${item.data.value}">${item.data.text}</span>`;
+       });

        selectedHtml = selectedHtml || this.placeholder;
      } else {
        selectedHtml = `${this.selectedOptions.length} ${this.selectedtext}`;
      }

-     this.dropdown.querySelector(".multiple-options").innerHTML = selectedHtml;
+     let options = this.dropdown.querySelector(".multiple-options");
+     if (options) {
+       options.innerHTML = selectedHtml;
+       options.addEventListener("click", (e) => this.#multipleListRemove(e));
+     }
    } else {
      const text = this.selectedOptions.length > 0 ? this.selectedOptions[0].data.text : this.placeholder;

      this.dropdown.querySelector(".current").textContent = text;
    }
  }
```

:335
```diff
  #renderItems() {
    const ul = this.dropdown.querySelector("ul");

    this.options.forEach((item) => {

-     if(this.multiple && this.config.showSelectedItems){
-       this.#multipleListAdd(item);
-     }

      ul.appendChild(this.#renderItem(item));
    });
  }
```

:405
```diff
  #onDropdownItemClicked(option, e) {
    const optionEl = e.target;

    if (hasClass(optionEl, "disabled")) {
      return;
    }

    if (this.multiple) {
      let selected;

      if (hasClass(optionEl, "selected")) {
        selected  = false;

        removeClass(optionEl, "selected");

        // Update Selected Options
        this.selectedOptions = this.selectedOptions.filter(
          (item) => item.data !== option.data
        );
      } else {
        selected  = true;

        addClass(optionEl, "selected");

        // Update Selected Options
        this.selectedOptions.push(option);      
      }

      // Update option 
      option.data.selected        = selected;
      option.attributes.selected  = selected;
    } else {      
      // Mark all dropdown options as unselected
      this.dropdown.querySelectorAll('li.selected').forEach((li) => removeClass(li, "selected"));

      // add the selected class to the current
      addClass(optionEl, "selected");

      // Update Selected Options Attribute
      this.selectedOptions = [option];

      // Update option properties 
      let prevSelected  = this.options.find(item => item.attributes.selected);
      if(prevSelected){
        prevSelected.data.selected        = false;
        prevSelected.attributes.selected  = false;
      }

      option.data.selected        = true;
      option.attributes.selected  = true;
    }

    this.#renderSelectedItems();

    this.#syncSelectValue();

-   this.#syncSelectionList();
  }
```

fixed match value instead of match text
: 437
```diff
  #syncSelectValue() {
    const select    = this.el;

    if (this.selectedOptions.length > 0) {
      select.value = this.selectedOptions[0].data.value;
    }else{
      // no value selected
      select.value = '';
      select.selectedIndex = -1;
    }

    this.options.forEach(item =>{
-     let matchingOption = Array.from(select.options).find(option => {
-       const a = String(option.dataset.display || option.textContent).trim().toLowerCase();
-       const b = String(item.data.text).trim().toLowerCase();
-       return a === b;
-     });
+     let matchingOption = Array.from(select.options).find(option => {
+       const a = String(option.value).trim().toLowerCase();
+       const b = String(item.data.value).trim().toLowerCase();
+       return a === b;
+     });

      if(matchingOption == undefined){
-       matchingOption = Array.from(select.options).find(option => {
-         const a = String(option.value).trim().toLowerCase();
-         const b = String(item.data.value).trim().toLowerCase();
-         return a === b;
-       });
+       matchingOption = Array.from(select.options).find(option => {
+         const a = String(option.dataset.display || option.textContent).trim().toLowerCase();
+         const b = String(item.data.text).trim().toLowerCase();
+         return a === b;
+       });
      }
```

:553 remove
```js
  /*
    Syncs the selected list with the dropdown
  */
  #syncSelectionList(){
    if(!this.config.showSelectedItems){
      return;
    }

    this.#removeSelectionList();

    // Update multiple list
    this.selectedOptions.forEach( item =>{
      this.#multipleListAdd(item);
    });
  }
```

:684 remove
```js
  #multipleListAdd(option) {
    if(!this.multiple || option.data.disabled || option.data.value == "" || !option.attributes.selected){
      return;
    }

    // Create the list
    if(this.selectionList == null){
      this.selectionList	 		= document.createElement('ul');
      this.selectionList.classList.add('select-selection-list');

      this.el.after(this.selectionList);
    }
    
    // Option is already in the list
    else if(this.selectionList.querySelector(`[data-value="${option.data.value}"]`) != null){
      return;
    }

    // Create a list element
    let li	 		= document.createElement('li');
    li.classList.add('select-selection');

    li.dataset.value = option.data.value;

    // Create a remove button
    let button  = document.createElement('button');
    button.classList.add('small', 'remove-select-selection');

    let span  = document.createElement('span');
    span.classList.add('remove-select-selection');
    span.textContent  = 'x';

    button.appendChild(span);

    li.appendChild(button);

    // Add the option text
    span  = document.createElement('span');
    span.classList.add('selected-name');
    span.textContent  = option.data.text;

    li.appendChild(span);

    this.selectionList.appendChild(li);

    // Add listener
    li.querySelectorAll('.remove-select-selection').forEach(el=> el.addEventListener("click", this.#multipleListRemove.bind(this)));
  }
```

```diff
- #multipleListRemove(target) {
+ #multipleListRemove(e) {

-   if(this.selectionList == null){
-     return;
-   }

-   if(target.target != null){
-     target  = target.target;
-   }

-   // Close button not clicked
-   if(target.matches == undefined || !target.matches('.remove-select-selection')){
-     return;
-   }

-   // Find the dropdown item and click it
-   let parent  = target.closest('li.select-selection');
+   let value = e.target.dataset?.value;

+   if (value) { 
-   let el  = this.options.find(item => item.data.value === parent.dataset.value).element;
+     let el = this.options.find(item => item.data.value == value).element;

      // only click when currently selected
      if(el && el.matches('.selected')){
        el.click();
      }
+   }
  }
```

## nice-select2.scss
:12
```diff
  $gray_dark: #444 !default;
- $gray: #999 !default;
- $gray_light: #e8e8e8 !default;
- $gray_lighter: #f6f6f6 !default;
- $primary_light: $gray !default;
- $arrow_color: $gray !default;
```

```diff
  // Style the dropdown
  .nice-select {
    -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
-   background-color: #fff;
+   background-color: var(--bs-body-bg);
    border-radius: $input_border_radius;
-   border: solid 1px $gray_light;
+   border: solid 1px var(--bs-border-color);
    box-sizing: border-box;
    clear: both;
    cursor: pointer;
    display: block;
-   float: left;
-   width: fit-content;
    font-family: inherit;
-   font-size: $font_size;
    font-weight: normal;
-   height: $input_height;
-   line-height: $input_height - 2;
    outline: none;
    padding-left: $dropdown_padding;
    padding-right: $dropdown_padding + 12;
    position: relative;
    text-align: left !important;
    transition: all 0.2s ease-in-out;
    user-select: none;
    white-space: nowrap;

-   &:hover {
+   &:hover, &:active, &:focus {
-       border-color: darken($gray_light, 5%);
-   }
-   &:active, &:focus {
-       border-color: $primary_light;
+       border-color: var(--bs-border-color)
    }
    // Arrow
    &:after {
-       border-bottom: 2px solid $arrow_color;
+       border-bottom: 2px solid var(--bs-body-color);
-       border-right: 2px solid $arrow_color;
+       border-right: 2px solid var(--bs-body-color);
        content: '';
        display: block;
-       height: 5px;
+       height: 8px;
-       margin-top: -4px;
+       margin-top: -5px;
        pointer-events: none;
        position: absolute;
-       right: 12px;
+       right: 15px;
        top: 50%;
        transform-origin: 66% 66%;
        transform: rotate(45deg);
        transition: all 0.15s ease-in-out;
-       width: 5px;
+       width: 8px;
    }
    &.open {
-     border-color: $primary_light;
+     border-color: var(--bs-border-color);
      &:after {
        transform: rotate(-135deg);
      }
      .nice-select-dropdown {
        opacity: 1;
        pointer-events: auto;
        transform: scale(1) translateY(0);
      }
    }
```
:78
```diff
  &.disabled {
-   border-color: lighten($gray_light, 2%);
+   border-color: var(--bs-secondary-bg);
+   background-color: var(--bs-secondary-bg);
-   color: $gray;
    pointer-events: none;
    &:after {
-     border-color: lighten($arrow_color, 20%);
+     border-color: var(--bs-secondary-color);
    }
  }
```
:116
```diff
  .nice-select-dropdown{
    margin-top: 4px;
-   background-color: #fff;
+   background-color: var(--bs-body-bg);
    border-radius: $input_border_radius;
    box-shadow: 0 0 0 1px rgba($gray_dark, .11);

    pointer-events: none;
    position: absolute;
    top: 100%; left: 0;
    transform-origin: 50% 0;
-   transform: scale(.75) translateY(- math.div(-$input-height, 2));
+   transform: scale(.75) translateY(- math.div(-$input_height, 2));
    transition: all .2s cubic-bezier(0.5, 0, 0, 1.25), opacity .15s ease-out;
    z-index: 9;
    opacity: 0;
  }
```

:124
```diff
- .has-multiple {
+ &.has-multiple {
    white-space: inherit;
    height: auto;
-   padding: 7px 12px;
    min-height: 36px;
    line-height: 22px;

    span.current {
-     border: 1px solid #CCC;
+     border: 1px solid var(--bs-border-color);
-     background: #EEE;
+     background: var(--bs-body-bg);
      padding: 0 10px;
      border-radius: 3px;
      display: inline-block;
      line-height: 24px;
      font-size: 14px;
      margin-bottom: 3px;
      margin-right: 3px;
+     box-shadow: 3px 3px var(--bs-primary-bg-subtle);

+     &:hover {
+       color: var(--bs-body-bg);
+       background-color: var(--bs-secondary-color)
+     }
    }

    .multiple-options {
      display: block;
      line-height: 24px;
      padding: 0;
    }
  }
```

:205
```diff
    span.current {
-     border: 1px solid #CCC;
+     border: 1px solid var(--bs-border-color);
-     background: #EEE;
+     background: var(--bs-body-bg);
      padding: 0 10px;
      border-radius: 3px;
```

:145
```diff
  .option {
    cursor: pointer;
    font-weight: 400;
-   line-height: $input_height + 2;
+   line-height: 1.5;
    list-style: none;
    outline: none;
    padding-left: $dropdown_padding;
    padding-right: $dropdown_padding + 11;
    text-align: left;
-   transition: all 0.2s;
+   &.selected.focus:not(:hover) {
+     color: var(--bs-emphasis-color);
+   }
    &:hover, &.focus, &.selected.focus {
+     color: var(--bs-body-bg);
-     background-color: $gray_lighter;
+     background-color: var(--bs-secondary-color)
    }
    &.selected {
      font-weight: bold;
    }
    &.disabled {
      background-color: transparent;
-     color: $gray;
+     color: var(--bs-secondary-color);
      cursor: default;
    }
  }
```

:232
```diff
  .nice-select-search {
    box-sizing: border-box;
-   background-color: #fff;
+   background-color: var(--bs-body-bg);
-   border: 1px solid $gray_light;
+   border: 1px solid var(--bs-border-color);
    border-radius: 3px;
-   color: $gray_dark;
+   color: var(--bs-body-color);
    display: inline-block;
    vertical-align: middle;
```

:248
```diff
  .hidden-select{
    opacity: 0;
    width: 0;
    padding: 0;
+   border: 0;
    height: 0;
    font-size: 0;
    min-height: auto;
}
```

add
```scss
// specific rounded corners fixed for "nice-select2 in .form-control" in .input-group
.input-group {
  > div:not(:last-child) > .form-control {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }
  > div:not(:first-child) > .form-control {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
}
```
