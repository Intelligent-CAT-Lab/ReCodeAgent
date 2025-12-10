use textrank::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_on_single_thread() {
    let raw_text = "Over the past fortnight we asked you to nominate your top extensions for the GNOME desktop. And you did just that. Having now sifted through the hundreds of entries, we're ready to reveal your favourite GNOME Shell extensions. GNOME 3 (which is more commonly used with the GNOME Shell) has an extension framework that lets developers (and users) extend, build on, and shape how the desktop looks, acts and functions. Dash to Dock takes the GNOME Dash — this is the 'favourites bar' that appears on the left-hand side of the screen in the Activities overlay — and transforms it into a desktop dock. And just like Plank, Docky or AWN you can add app launchers, rearrange them, and use them to minimise, restore and switch between app windows. Dash to Dock has many of the common \"Dock\" features you'd expect, including autohide and intellihide, a fixed-width mode, adjustable icon size, and custom themes. My biggest pet peeve with GNOME Shell is its legacy app tray that hides in the bottom left of the screen. All extraneous non-system applets, indicators and tray icons hide down here. This makes it a little harder to use applications that rely on a system tray presence, like Skype, Franz, Telegram, and Dropbox. TopIcons Plus is the quick way to put GNOME system tray icons back where they belong: on show and in reach. The extension moves legacy tray icons from the bottom left of Gnome Shell to the right-hand side of the top panel. A well-stocked settings panel lets you adjust icon opacity, color, padding, size and tray position. Dive into the settings to adjust the sizing, styling and positioning of icons. Like the popular daily stimulant of choice, the Caffeine GNOME extension keeps your computer awake. It couldn't be simpler to use: just click the empty mug icon. An empty cup means you're using normal auto suspend rules – e.g., a screensaver – while a freshly brewed cup of coffee means auto suspend and screensaver are turned off. The Caffeine GNOME extension supports GNOME Shell 3.4 or later. Familiar with applications like Guake and Tilda? If so, you'll instantly see the appeal of the (superbly named) Drop Down Terminal GNOME extension. When installed just tap the key above the tab key (though it can be changed to almost any key you wish) to get instant access to the command line. Want to speed up using workspaces? This simple tool lets you do just that. Once installed you can quickly switch between workspaces by scrolling over the top panel - no need to enter the Activities Overlay!";

    let mut tr = NewTextRank();
    let rule = NewDefaultRule();
    let language = NewDefaultLanguage();
    let algorithm_def = NewDefaultAlgorithm();

    tr.Populate(raw_text, &language, &rule);
    tr.Ranking(&algorithm_def);

    assert_the_gnome_test_text_default(&tr);

    let algorithm_chain = NewChainAlgorithm();
    tr.Ranking(&algorithm_chain);

    assert_the_gnome_test_text_chain(&tr);
}

#[test]
fn test_on_multi_thread() {
    let (tx, rx) = mpsc::channel::<String>();
    let mut tr = NewTextRank();

    let raw_texts = vec![
        "Over the past fortnight we asked you to nominate your top extensions for the GNOME desktop.".to_string(),
        "And you did just that. Having now sifted through the hundreds of entries, we're ready to reveal your favourite GNOME Shell extensions. GNOME 3 (which is more commonly used with the GNOME Shell) has an extension framework that lets developers (and users) extend, build on, and shape how the desktop looks, acts and functions.".to_string(),
        "Dash to Dock takes the GNOME Dash — this is the 'favourites bar' that appears on the left-hand side of the screen in the Activities overlay — and transforms it into a desktop dock. And just like Plank, Docky or AWN you can add app launchers, rearrange them, and use them to minimise, restore and switch between app windows. Dash to Dock has many of the common \"Dock\" features you'd expect, including autohide and intellihide, a fixed-width mode, adjustable icon size, and custom themes.".to_string(),
        "My biggest pet peeve with GNOME Shell is its legacy app tray that hides in the bottom left of the screen. All extraneous non-system applets, indicators and tray icons hide down here. This makes it a little harder to use applications that rely on a system tray presence, like Skype, Franz, Telegram, and Dropbox. TopIcons Plus is the quick way to put GNOME system tray icons back where they belong: on show and in reach. The extension moves legacy tray icons from the bottom left of Gnome Shell to the right-hand side of the top panel. A well-stocked settings panel lets you adjust icon opacity, color, padding, size and tray position. Dive into the settings to adjust the sizing, styling and positioning of icons. Like the popular daily stimulant of choice, the Caffeine GNOME extension keeps your computer awake.".to_string(),
        "It couldn't be simpler to use: just click the empty mug icon. An empty cup means you're using normal auto suspend rules – e.g., a screensaver – while a freshly brewed cup of coffee means auto suspend and screensaver are turned off. The Caffeine GNOME extension supports GNOME Shell 3.4 or later. Familiar with applications like Guake and Tilda? If so, you'll instantly see the appeal of the (superbly named) Drop Down Terminal GNOME extension. When installed just tap the key above the tab key (though it can be changed to almost any key you wish) to get instant access to the command line. Want to speed up using workspaces? This simple tool lets you do just that. Once installed you can quickly switch between workspaces by scrolling over the top panel - no need to enter the Activities Overlay!".to_string(),
    ];

    // Send all texts
    let sender_thread = thread::spawn(move || {
        for raw_text in raw_texts {
            tx.send(raw_text).unwrap();
        }
    });

    // Process texts as they arrive
    let mut count = 0;
    while count < 5 {
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(raw_text) => {
                let rule = NewDefaultRule();
                let language = NewDefaultLanguage();
                let algorithm = NewDefaultAlgorithm();

                tr.Populate(&raw_text, &language, &rule);
                tr.Ranking(&algorithm);
                count += 1;
            }
            Err(_) => break,
        }
    }

    sender_thread.join().unwrap();

    assert_the_gnome_test_text_default(&tr);
}

fn assert_the_gnome_test_text_default(text_rank: &TextRank) {
    let most_populars = vec![
        "gnome shell",
        "icons tray",
        "extension gnome",
        "gnome caffeine",
        "key tab",
        "key changed",
        "overlay activities",
        "suspend auto",
        "dock dash",
    ];

    let phrases = text_rank.FindPhrases();
    let max = most_populars.len() - 1;

    for i in 0..max {
        let found = {
            let ph = &phrases[i];
            let mut found_val = false;
            for popular in &most_populars {
                let expression = format!("{} {}", ph.left, ph.right);
                if expression == *popular {
                    found_val = true;
                    break;
                }
            }
            found_val
        };

        assert_eq!(true, found);
    }

    let rank_for_check = text_rank.GetRankData();

    assert_eq!(1.0, phrases[0].weight);
    assert_eq!(5, phrases[0].qty);
    assert_eq!("gnome", phrases[0].left);
    assert_eq!("shell", phrases[0].right);
    assert_eq!(phrases[0].left_id, rank_for_check.word_val_id[&phrases[0].left]);
    assert_eq!(phrases[0].right_id, rank_for_check.word_val_id[&phrases[0].right]);

    let single_words = text_rank.FindSingleWords();

    assert_eq!("gnome", single_words[0].word);
    assert_eq!(1.0, single_words[0].weight);
    assert_eq!(12, single_words[0].qty);
    assert_eq!(single_words[0].id, rank_for_check.word_val_id[&single_words[0].word]);

    let sentences_by_qty_weight = text_rank.FindSentencesByWordQtyWeight(6);

    assert_eq!(6, sentences_by_qty_weight.len());
    assert_eq!(0, sentences_by_qty_weight[0].id);
    assert_eq!(2, sentences_by_qty_weight[1].id);
    assert_eq!(3, sentences_by_qty_weight[2].id);
    assert_eq!(4, sentences_by_qty_weight[3].id);
    assert_eq!(7, sentences_by_qty_weight[4].id);
    assert_eq!(sentences_by_qty_weight[4].value, rank_for_check.sentence_map[&sentences_by_qty_weight[4].id]);

    let sentences_by_rel_weight = text_rank.FindSentencesByRelationWeight(6);

    assert_eq!(6, sentences_by_rel_weight.len());
    assert_eq!(2, sentences_by_rel_weight[0].id);
    assert_eq!(3, sentences_by_rel_weight[1].id);
    assert_eq!(7, sentences_by_rel_weight[2].id);
    assert_eq!(11, sentences_by_rel_weight[3].id);
    assert_eq!(19, sentences_by_rel_weight[4].id);
    assert_eq!(sentences_by_rel_weight[4].value, rank_for_check.sentence_map[&sentences_by_rel_weight[4].id]);

    let sentences_by_phrase = text_rank.FindSentencesByPhraseChain(&vec![
        "gnome".to_string(),
        "shell".to_string(),
        "extension".to_string(),
    ]);

    assert_eq!(3, sentences_by_phrase[0].id);
    assert_eq!(19, sentences_by_phrase[1].id);
    assert_eq!(sentences_by_phrase[1].value, rank_for_check.sentence_map[&sentences_by_phrase[1].id]);

    let sentence_id_start = 10;
    let found_sentences = text_rank.FindSentencesFrom(sentence_id_start, 3);

    assert_eq!(sentence_id_start, found_sentences[0].id);
    assert_eq!(sentence_id_start + 1, found_sentences[1].id);
    assert_eq!(sentence_id_start + 2, found_sentences[2].id);
    assert_eq!(3, found_sentences.len());
    assert_eq!(found_sentences[0].value, rank_for_check.sentence_map[&found_sentences[0].id]);
}

fn assert_the_gnome_test_text_chain(text_rank: &TextRank) {
    let most_populars = vec![
        "gnome shell",
        "extension gnome",
        "icons tray",
        "gnome caffeine",
        "key tab",
        "key changed",
        "overlay activities",
        "suspend auto",
        "dock dash",
    ];

    let phrases = text_rank.FindPhrases();
    let max = most_populars.len() - 1;

    for i in 0..max {
        let found = {
            let ph = &phrases[i];
            let mut found_val = false;
            for popular in &most_populars {
                let expression = format!("{} {}", ph.left, ph.right);
                if expression == *popular {
                    found_val = true;
                    break;
                }
            }
            found_val
        };

        assert_eq!(true, found);
    }
}
