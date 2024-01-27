use yew::prelude::*;

#[function_component(MarkdownLike)]
pub fn markdown_like() -> Html {
    html! {
        <div
            style="
            -webkit-text-size-adjust: 100%;
            color: #373D49;
            box-sizing: border-box;
            font-family: Georgia,Cambria,serif;
            height: 100%;
            font-weight: 400;
            line-height: 2rem;
            max-width: 1024px;
            margin: 0 auto;
            overflow: auto;
            padding: 2%;
            font-size: 1rem;
            "
        >
            <h3
            class="code-line"
            data-line-start="0"
            data-line-end="1"
            style="
            -webkit-text-size-adjust: 100%;
            color: #ffffa2;
            box-sizing: border-box;
            font-family: \"Source Sans Pro\",\"Helvetica Neue\",Helvetica,Arial,sans-serif;
            font-feature-settings: 'dlig' 1,'liga' 1,'lnum' 1,'kern' 1;
            font-style: normal;
            font-weight: 600;
            margin-top: 0;
            line-height: 3rem;
            font-size: 1.6457143rem;
            margin-bottom: .07599rem;
            padding-top: .92401rem;
            "
            >
                <a
                id="commit_string_0"
                href=""
                style="
                -webkit-text-size-adjust: 100%;
                font-family: \"Source Sans Pro\",\"Helvetica Neue\",Helvetica,Arial,sans-serif;
                font-feature-settings: 'dlig' 1,'liga' 1,'lnum' 1,'kern' 1;
                font-style: normal;
                font-weight: 600;
                line-height: 3rem;
                font-size: 1.6457143rem;
                box-sizing: border-box;
                background: 0 0;
                cursor: pointer;
                color: #35D7BB;
                text-decoration: none;
                "
                ></a>{"commit string"}
            </h3>
            <pre
            style="
            -webkit-text-size-adjust: 100%;
            font-weight: 400;
            box-sizing: border-box;
            display: block;
            margin: 0 0 10px;
            word-break: break-all;
            word-wrap: break-word;
            color: #333;
            border: 1px solid #ccc;
            border-radius: 4px;
            overflow: auto;
            font-family: monospace,monospace;
            padding: .66001rem 9.5px 9.5px;
            line-height: 2rem;
            background: linear-gradient(to bottom,#fff 0,#fff .75rem,#f5f7fa .75rem,#f5f7fa 2.75rem,#fff 2.75rem,#fff 4rem);
            background-size: 100% 4rem;
            border-color: #D3DAEA;
            margin-bottom: 1.33999rem;
            font-size: 1rem;
            padding-top: .66001rem;
            "
            >
                <code
                class="has-line-data"
                data-line-start="2"
                data-line-end="4"
                style="
                -webkit-text-size-adjust: 100%;
                font-weight: 400;
                word-break: break-all;
                word-wrap: break-word;
                line-height: 2rem;
                box-sizing: border-box;
                font-family: monospace,monospace;
                padding: 0;
                font-size: inherit;
                color: inherit;
                white-space: pre-wrap;
                background-color: transparent;
                border-radius: 0;
                "
                >
                    {"commit string sss"}
                </code>
            </pre>
        </div>
    }
}
