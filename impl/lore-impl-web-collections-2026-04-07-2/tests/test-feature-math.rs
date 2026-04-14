//! test it with
//! 
//! ```bash
//! cargo test --features math --test test-feature-math -- --nocapture
//! ```

use lore_web_collections_core::LineType;
use lore_impl_web_collections_2026_04_07_2::lore_html::LoreHtml;
use lore_impl_web_collections_2026_04_07_2::parser::Parser;
use lore_impl_web_collections_2026_04_07_2::impl_context::ImplContext;

#[test]
fn test_feature_math_enabled() {
    let impl_context = ImplContext {
        title: "Math Test Page",
        link_base: "https://www.example.com/",
        css_url: "https://fleetinglore.github.io/css/style.css",
    };
    let parser = Parser { impl_context: &impl_context };

    let ir_nodes = [
        LineType::Comment("This page has math rendering enabled".to_string()),
        LineType::DomainTitle("Integral Table".to_string()),
        LineType::Atom("For $\\int x^2 dx = \\frac{x^3}{3} + C$".to_string()),
        LineType::Atom("Definite integral: $\\int_0^1 x^2 dx = \\frac{1}{3}$".to_string()),
        LineType::Atom("Display formula: $$\\int_{-\\infty}^{\\infty} e^{-x^2} dx = \\sqrt{\\pi}$$".to_string()),
        LineType::Placeholder,
        LineType::LoreLink(
            "View more integrals".to_string(),
            "more_integrals".to_string()
        )
    ];

    let target: LoreHtml = parser.parse(&ir_nodes, &impl_context);
    let target: String = target.into();
    
    #[cfg(feature = "math")]
    {
        assert!(target.contains("MathJax"), "Should contain MathJax configuration");
        assert!(target.contains("cdn.jsdelivr.net/npm/mathjax@3"), "Should contain MathJax CDN");
        assert!(target.contains(r"\int x^2 dx"), "Should contain integral symbol");
        println!("Math feature enabled, HTML contains MathJax script\n");
    }
    
    #[cfg(not(feature = "math"))]
    {
        assert!(!target.contains("MathJax"), "Should not contain MathJax configuration");
        assert!(!target.contains("cdn.jsdelivr.net/npm/mathjax"), "Should not contain MathJax CDN");
        println!("Math feature disabled, HTML does not contain MathJax script\n");
    }
    
    println!("{}", target);
}

#[test]
fn test_feature_math_without_formulas() {
    let impl_context = ImplContext {
        title: "No Formulas Page",
        link_base: "https://www.example.com/",
        css_url: "https://fleetinglore.github.io/css/style.css",
    };
    let parser = Parser { impl_context: &impl_context };

    let ir_nodes = [
        LineType::DomainTitle("Plain Document".to_string()),
        LineType::Atom("This is plain text without mathematical formulas.".to_string()),
        LineType::LoreLink(
            "Back to Home".to_string(),
            "index".to_string()
        )
    ];

    let target: LoreHtml = parser.parse(&ir_nodes, &impl_context);
    let target: String = target.into();
    
    #[cfg(feature = "math")]
    {
        assert!(target.contains("MathJax"), "MathJax should be included when math feature is enabled");
        println!("Math feature enabled, MathJax script included even without formulas\n");
    }
    
    #[cfg(not(feature = "math"))]
    {
        assert!(!target.contains("MathJax"), "MathJax should not be included when math feature is disabled");
        println!("Math feature disabled, page remains lightweight\n");
    }
    
    println!("{}", target);
}