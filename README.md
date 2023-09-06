<!-- Arabic Section -->
<div dir=rtl>
<h1>
تجربة استخدام اطار واجهات ياو مع اطار فلوبيت للواجهات
</h1>
<p>
هذا مثال وتجربة لاستخدام اطار واجهات الويب المبرمج بلغة روست واسمه ياو، مع اطار فلوبيت التنسيقي والتصميمي

ماهو فلوبيت؟
اسمه بالانجليزي "flowbite" وهو اطار لتشكيل هياكل الويب باستخدام تيلوايند "tailwindcss"
ما يميز فلوبيت هو انه يختصر الوقت في ايجاد مكونات مترابطة ومتناسقة ومختبرة فتوفر الوقت والجهد.
 
</p>

<ul>الأهداف:
    <li>استخدام روست في برمجة واجهات الويب</li>
    <li>اردت تجربة تصميم موقع بالروست كاملا من ناحية النظام الداخلي والواجهات</li>
    <li>وجدت سهولة في اطار فلوبيت واردت اختباره هنا</li>
</ul

<p>
ملحوظة: في الحقيقة انا طموح جدا لاستخدام روست في صناعة برمجيات الويب بشكل كامل من نظام داخلي وخارجي، ما زال هناك الكثير في هذا المثال وارحب بمشاركاتكم
</p

</div>


<!-- English Section -->

# Try and experimenting using Flowbite framework with Yew

This is an illustraiton to show how to use [Flowbite](flowbite.com/) framework with Yew framework.


**My goals**
- Using Rust in backendend and frontend
- Develop a web app with support to RTL (Arabic) and LTR (English)
- I like FLowbite framework and I want to use it with Yew to minimize effort.

---

**Screen sample**
![Screenshot from 2023-09-06 20-34-18](https://github.com/moaz-mokhtar/try_yew_with_flowbite/assets/5870208/362bce38-859b-4348-a873-deb743f2ee14)



---

My feedback till now:
- Flowbite can be used with Yew, but need some customization because not all scripts are done.
- For example I found that in `index.html` the tags`link` and `script` which not have `data-trunk` not working properly:
    ```html
    <!DOCTYPE html>
    <html>

    <head>
        <meta charset="utf-8" />
        <title>Yew App with Flowbite</title>
        <link data-trunk rel="tailwind-css" href="tailwind.css" />
        <link href="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.css" rel="stylesheet" />
    </head>

    <body>

        <div id="root"></div>

        <script src="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.js"></script>
    </body>

    </html>
    ```
  - In this example Dark mode should be active, but it is not.
  - It is fixed by pointing to the css and js files locally as below:
    ```html
    <!DOCTYPE html>
        <html>

        <head>
            <meta charset="utf-8" />
            <title>Yew App with Flowbite</title>
            <link data-trunk rel="tailwind-css" href="tailwind.css" />
            <link href="assets/flowbite.min.css" rel="stylesheet" />
        </head>

        <body>
            <script src="assets/flowbite.min.js"></script>
        </body>

    </html>
    ```

- I'm not sure if using JS callback will resolve other issues or not, like exmample: [yew / exmaples / js_callback](https://github.com/yewstack/yew/tree/master/examples/js_callback)


Finally, I would like to say I'm very passionate about frontend solutions with Rust and still alot of work needed to improve this.
