-- Your SQL goes here
CREATE TABLE coffees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coffee_name TEXT NOT NULL,
    additions TEXT[] NOT NULL,
    description TEXT NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    image_path TEXT NOT NULL
);

INSERT INTO coffees (additions, coffee_name, description, price, image_path) VALUES 
(ARRAY['TRADICIONAL'], 'Expresso Tradicional', 'O tradicional café feito com água quente e grãos moídos', 9.9, 'coffee-expresso.png'),
(ARRAY['TRADICIONAL'], 'Expresso Americano', 'Expresso diluído, menos intenso que o tradicional', 9.9, 'coffee-american.png'),
(ARRAY['TRADICIONAL'], 'Expresso Cremoso', 'Café expresso tradicional com espuma cremosa', 9.9, 'coffee-expresso-cremoso.png'),
(ARRAY['TRADICIONAL', 'GELADO'], 'Expresso Gelado', 'Bebida preparada com café expresso e cubos de gelo', 9.9, 'coffee-gelado.png'),
(ARRAY['TRADICIONAL', 'COM LEITE'], 'Café com Leite', 'Meio a meio de expresso tradicional com leite vaporizado', 9.9, 'coffee-milk.png'),
(ARRAY['TRADICIONAL', 'COM LEITE'], 'Latte', 'Uma dose de café expresso com o dobro de leite e espuma cremosa', 9.9, 'coffee-late.png'),
(ARRAY['TRADICIONAL', 'COM LEITE'], 'Capuccino', 'Bebida com canela feita de doses iguais de café, leite e espuma', 9.9, 'coffee-capuccino.png'),
(ARRAY['TRADICIONAL', 'COM LEITE'], 'Macchiato', 'Café expresso misturado com um pouco de leite quente e espuma', 9.9, 'coffee-machiato.png'),
(ARRAY['TRADICIONAL', 'COM LEITE'], 'Mocaccino', 'Café expresso com calda de chocolate, pouco leite e espuma', 9.9, 'coffee-mocaccino.png'),
(ARRAY['ESPECIAL', 'COM LEITE'], 'Chocolate Quente', 'Bebida feita com chocolate dissolvido no leite quente e café', 9.9, 'coffee-chocolate-quente.png'),
(ARRAY['ESPECIAL', 'ALCOÓLICO', 'GELADO'], 'Cubano', 'Drink gelado de café expresso com rum, creme de leite e hortelã', 9.9, 'coffee-cuban.png'),
(ARRAY['ESPECIAL'], 'Havaiano', 'Bebida adocicada preparada com café e leite de coco', 9.9, 'coffee-havaiano.png'),
(ARRAY['ESPECIAL'], 'Árabe', 'Bebida preparada com grãos de café árabe e especiarias', 9.9, 'coffee-arabe.png'),
(ARRAY['ESPECIAL', 'ALCOÓLICO'], 'Irlandês', 'Bebida a base de café, uísque irlandês, açúcar e chantilly', 9.9, 'coffee-irlandes.png');