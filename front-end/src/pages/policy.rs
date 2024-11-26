use dioxus::prelude::*;

#[component]
pub fn Policy() -> Element {
    rsx! {
        div { class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
            div { class: "content",
            h1 { class: "text-3xl font-bold mb-4", "Terms and Conditions" }
            p { class: "text-sm text-gray-500 mb-6", "Last Updated: December 16, 2020" }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Domain Specification" }
                p { class: "mb-4",
                    "Unless otherwise noted, all references to DanceBattleZ include ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " and DanceBattleZ."
                }
                p { class: "mb-4",
                    "All references to DanceBattleZ also include the DanceBattleZ mobile app for iOS and Android, which is a custom browser designed to navigate directly to ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " when launched."
                }
                p { class: "mb-4", "The DanceBattleZ website is a dance battle site." }
                p { class: "mb-4",
                    "By observing or interacting with the DanceBattleZ website, you consent to the data practices described herein."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Terms and Conditions" }
                p { class: "mb-4",
                    "The ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " website (the \"Site\") consists of various web pages operated by DanceBattleZ Inc. (\"DanceBattleZ\"). The Site is offered to you conditioned on your acceptance without modification of the terms, conditions, and notices contained herein (the \"Terms\")."
                }
                p { class: "mb-4",
                    "Your observation of, or interaction with ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " constitutes your agreement to all such Terms."
                }
                p { class: "mb-4", "Please read these terms carefully, and keep a copy of them for your reference." }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Privacy Policies" }
                p { class: "mb-4",
                    "Protecting your private information is our priority. The Statements of Privacy contained herein apply to ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " and DanceBattleZ and governs data collection and usage."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Persons Under Thirteen" }
                p { class: "mb-4", "DanceBattleZ does not knowingly collect personal information from persons under the age of thirteen." }
                p { class: "mb-4",
                    "If you are under the age of thirteen, you must ask your parent or guardian for permission to observe or interact with this website."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "International Access" }
                p { class: "mb-4",
                    "The Service is controlled, operated, and administered by DanceBattleZ from our offices within the USA. If you access the Service from a location outside the USA, you are responsible for compliance with all local laws. You agree that you will not access, observe, or utilize the DanceBattleZ content in any manner prohibited by any applicable laws, restrictions, or regulations."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "App Contents" }
                p { class: "mb-4",
                    "The purpose of the Site is to bring greater recognition to the performing arts by hosting dance video battles. Please utilize as intended."
                }
                p { class: "mb-4",
                    "Outside of your account info, posts, votes, and branded content, the content on this Site belongs to DanceBattleZ. This includes statistics pertaining to votes, battlers, and battles, and the Site's design and processes."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "No Unlawful or Prohibited Actions / Intellectual Property" }
                p { class: "mb-4",
                    "You are granted a non-exclusive, non-transferable, revocable license to access and utilize ",
                    a { href: "http://www.dancebattlez.com", class: "text-blue-600 hover:underline", "www.dancebattlez.com" },
                    " strictly in accordance with these terms and conditions."
                }
                p { class: "mb-4",
                    "As a condition of your utilization of the Site, you warrant to DanceBattleZ that you will not utilize the Site for any purpose that is unlawful or prohibited by these Terms."
                }
                p { class: "mb-4",
                    "You may not utilize the Site in any manner which could damage, disable, overburden, or impair the Site or interfere with any other party's utilization and enjoyment of the Site."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Personal Information" }
                p { class: "mb-4",
                    "We do not collect any personal information about you unless you voluntarily provide it to us. However, you may be required to provide certain personal information when you utilize certain products or services on the Site, including registering for an account or submitting payment information."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-2", "Security" }
                p { class: "mb-4",
                    "DanceBattleZ secures your personal information from unauthorized access or disclosure. We utilize SSL protocols, obfuscation, and other methods to protect your data, but no transmission over the Internet is 100% secure."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Cookies and Trackers" }
                p { class: "",
                    "This Site utilizes first-party cookies to maintain sessions with logged-in account holders."
                }
                p { class: "mt-2",
                    "This Site may utilize Google Analytics and other trackers, along with their third-party cookies, to gain insight into the amount of traffic the Site is handling."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Right to Deletion" }
                p { class: "",
                    "You may delete all of your data at any time from within the settings page, except for the votes you cast. The votes will have any associated account information removed. For questions, contact us at ",
                    a { href: "mailto:neutron@dancebattlez.com", class: "text-blue-500 underline", "neutron@dancebattlez.com" },
                    "."
                }
                p { class: "mt-2",
                    "On receipt of a verifiable request, we will delete your personal information from our records and direct any service providers to do the same, except in certain cases such as completing transactions, detecting fraud, debugging errors, or complying with legal obligations."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "E-Mail Communications" }
                p { class: "",
                    "DanceBattleZ may contact you via email for announcements, promotional offers, alerts, confirmations, surveys, and other communications."
                }
                p { class: "mt-2",
                    "If you wish to stop receiving marketing emails, you can opt out by clicking the UNSUBSCRIBE button at the bottom of the emails."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Electronic Communications" }
                p { class: "",
                    "By visiting ",
                    a { href: "https://www.dancebattlez.com", class: "text-blue-500 underline", "www.dancebattlez.com" },
                    " or sending emails to DanceBattleZ, you consent to receive electronic communications and agree that all notices and communications provided electronically satisfy any legal requirements."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Changes to this Statement" }
                p { class: "",
                    "DanceBattleZ reserves the right to update this Privacy Statement from time to time. Your continued use of the Site constitutes your acceptance of these updates."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Contact Information" }
                p { class: "",
                    "For questions or concerns about this Privacy Statement or our privacy practices, contact us at ",
                    a { href: "mailto:neutron@dancebattlez.com", class: "text-blue-500 underline", "neutron@dancebattlez.com" },
                    "."
                }
            }

            section { class: "mb-8",
                h2 { class: "text-2xl font-semibold mb-4", "California Privacy Rights" }
                p { class: "",
                    "Under California Civil Code Section 1798.83, California residents are entitled to request and obtain information about how their data is shared with third parties for marketing purposes. For more details, please contact us."
                }
            }
        }
        }
    }
}
