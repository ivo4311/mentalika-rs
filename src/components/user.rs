use yew::prelude::*;

#[function_component]
pub fn UserCard() -> Html {
    html! {
        <div class="w3-white w3-text-grey w3-card-4">
            <UserAvatar/>
            <div class="w3-container">
                <p><i class="fa fa-home fa-fw w3-margin-right w3-large w3-text-teal"></i>{"Sofia, Bulgaria"}</p>
                <hr />
                <UserSkills/>
                <UserAchievements/>
            </div>
        </div>
    }
}

#[function_component]
fn UserAvatar() -> Html {
    html! {
        <div class="w3-display-container">
            <img src="img/mishe.jpg" style="width: 100%" alt="Avatar" />
            <div class="w3-display-bottomleft w3-container w3-text-black">
                <h2>{"Менталика мише?"}</h2>
            </div>
        </div>
    }
}

#[function_component]
fn UserSkills() -> Html {
    html! {
        <>
        <p class="w3-large">
            <b><i class="fa fa-asterisk fa-fw w3-margin-right w3-text-teal"></i>{"Умения (TODO)"}</b>
        </p>
        <p>{"Умножение"}</p>
        <div class="w3-light-grey w3-round-xlarge w3-small">
            <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{"100%"}</div>
        </div>
        <p>{"Деление"}</p>
        <div class="w3-light-grey w3-round-xlarge w3-small">
            <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{"100%"}</div>
        </div>
        <p>{"Събиране и Изваждане"}</p>
        <div class="w3-light-grey w3-round-xlarge w3-small">
            <div class="w3-container w3-center w3-round-xlarge w3-teal" style="width: 100%">{"100%"}</div>
        </div>
        <hr />
        </>
    }
}

#[function_component]
fn UserAchievements() -> Html {
    html! {
        <>
        <p class="w3-large">
            <b><i class="fa fa-solid fa-award fa-fw w3-margin-right w3-text-teal"></i>{"Заслуги (TODO)"}</b>
        </p>
        <hr />
        </>
    }
}
