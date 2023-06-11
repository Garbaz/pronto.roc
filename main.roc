platform "pronto"
    requires {} { main: { init : Init, draw : Draw } }
    exposes [Pronto]
    packages {}
    imports [Pronto.{ Init, Window, Draw }]
    provides [mainForHost]

mainForHost = main