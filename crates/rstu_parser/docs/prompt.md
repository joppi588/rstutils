Create a recursive descent parser for the tokens.
Start with the following rules (tokens in <>):
Sentence = <Word>| Sentence <Space> 
Heading = Sentence <NewLine> <HeadingUnderline> | <Heading> <NewLine> Sentence <NewLine> <HeadingUnderline>
Comment = <DoubleDot> <Space> Sentence
Directive = <DoubleDot> 