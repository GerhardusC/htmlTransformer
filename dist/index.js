const inputContainer = document.getElementById("input-container");
const outputContainer = document.getElementById("output-container");
const lowercaseBtn = document.getElementById("lowercase-button");
const uppercaseBtn = document.getElementById("uppercase-button");
const resetBtn = document.getElementById("reset-button");
const selectorInput = document.getElementById("selector-input");

if(
    !inputContainer ||
    !outputContainer ||
    !lowercaseBtn ||
    !uppercaseBtn ||
    !selectorInput
) {
    throw Error("ERR_FAILED_INIT: Page is not laid out correctly.");
}

lowercaseBtn.addEventListener("click", async function(_e) {
    outputContainer.innerText = "Loading";
    try {
        const transformed = await transformInputHtmlToCase("lowercase")
        outputContainer.innerText = transformed;
    } catch (e) {
        outputContainer.innerText = `Failed with error: ${e}`;
    }
})

uppercaseBtn.addEventListener("click", async function(_e) {
    outputContainer.innerText = "Loading";
    try {
        const transformed = await transformInputHtmlToCase("uppercase")
        outputContainer.innerText = transformed;
    } catch (e) {
        outputContainer.innerText = `Failed with error: ${e}`;
    }
})

resetBtn.addEventListener("click", function () {
    outputContainer.innerText = "";
    selectorInput.value = "";
    inputContainer.value = "";
})

/**
 * 
 * @param {string} html 
 * @param {"uppercase" | "lowercase"} targetCase 
 * @returns {Promise<string>}
 */
async function transformInputHtmlToCase(targetCase) {
    const inputHtml = inputContainer.value;
    const selector = selectorInput.value;

    if(!inputHtml) {
        throw Error("Input required.");
    }

    const res = await fetch("/transform", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            transform: targetCase,
            html: inputHtml,
            selector: selector === "" ? "p" : selector,
        })
    })
    return await res.text();
}
