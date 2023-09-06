use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="sticky bottom-0 bg-white p-2 rounded-none shadow mt-auto dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
            <div class="text-l text-gray-500 sm:text-center dark:text-gray-400">
                { "© 2023 Powered by " }
                <a href="https://yew.rs">{ "Yew" }</a>
                { " using " }
                <a href="https://flowbite.com">{ "Flowbite" }</a>
            </div>
        </footer>

    //     <footer
	// 	class="sticky bottom-0 bg-white rounded-none shadow mt-auto dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700"
	// >
	// 	<div class="w-full mx-auto max-w-screen-xl p-4 md:flex md:items-center md:justify-between">
	// 		<span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
	// 			{"© 2023 تم التطوير بواسطة معاذ بن مختار"}
	// 		</span>
	// 	</div>
	// </footer>
    }
}
