import unittest

from socha.api.plugin.penguins import *


class TestVector(unittest.TestCase):
    def setUp(self):
        self.vector1 = Vector(3, 4)
        self.vector2 = Vector(5, 12)
        self.vector3 = Vector(-3, -4)
        self.vector4 = Vector(3, 4)

    def test_magnitude(self):
        self.assertAlmostEqual(self.vector1.magnitude(), 5.0)
        self.assertAlmostEqual(self.vector2.magnitude(), 13.0)

    def test_dot_product(self):
        self.assertEqual(self.vector1.dot_product(self.vector2), 63)

    def test_cross_product(self):
        self.assertEqual(self.vector1.cross_product(self.vector2), 16)

    def test_scalar_product(self):
        self.assertEqual(self.vector1.scalar_product(2), Vector(6, 8))

    def test_addition(self):
        self.assertEqual(self.vector1.addition(self.vector3), Vector(0, 0))

    def test_subtraction(self):
        self.assertEqual(self.vector1.subtraction(self.vector3), Vector(6, 8))

    def test_get_arc_tangent(self):
        self.assertAlmostEqual(self.vector1.get_arc_tangent(), 0.9272952180016122)
        self.assertAlmostEqual(self.vector2.get_arc_tangent(), 1.176005207095135)

    def test_are_identically(self):
        self.assertTrue(self.vector1.are_identically(self.vector4))

    def test_are_equal(self):
        self.assertFalse(self.vector1.are_equal(self.vector2))

    def test_directions(self):
        self.assertEqual(self.vector1.directions,
                         [Vector(1, -1), Vector(-2, 0), Vector(1, 1), Vector(-1, 1), Vector(2, 0), Vector(-1, -1)])


class TestCoordinate(unittest.TestCase):
    def setUp(self):
        self.coordinate1 = Coordinate(3, 4)
        self.coordinate2 = Coordinate(5, 12)
        self.coordinate3 = Coordinate(-3, -4)
        self.coordinate4 = Coordinate(1, -1)
        self.coordinate5 = Coordinate(3, 4)
        self.vector1 = Vector(3, 4)
        self.vector2 = Vector(5, 12)
        self.vector3 = Vector(-3, -4)
        self.vector4 = Vector(1, -1)
        self.vector5 = Vector(3, 4)

    def test_to_vector(self):
        self.assertEqual(self.coordinate1.to_vector(), self.vector1)
        self.assertEqual(self.coordinate2.to_vector(), self.vector2)

    def test_distance(self):
        self.assertAlmostEqual(self.coordinate1.distance(self.coordinate2), 8.246211251235321)


class TestCartesianCoordinate(unittest.TestCase):
    def setUp(self):
        self.cartesian_coordinate1 = CartesianCoordinate(0, 4)
        self.cartesian_coordinate2 = CartesianCoordinate(5, 12)
        self.cartesian_coordinate3 = CartesianCoordinate(-3, -4)
        self.vector1 = Vector(3, 4)
        self.vector2 = Vector(5, 12)
        self.hex_coordinate1 = HexCoordinate(10, 12)
        self.index1 = 32
        self.index2 = -1

    def test_add_vector(self):
        self.assertEqual(self.cartesian_coordinate1.add_vector(self.vector1), CartesianCoordinate(3, 8))
        self.assertEqual(self.cartesian_coordinate2.add_vector(self.vector2), CartesianCoordinate(10, 24))

    def test_subtract_vector(self):
        self.assertEqual(self.cartesian_coordinate1.subtract_vector(self.vector1), CartesianCoordinate(-3, 0))
        self.assertEqual(self.cartesian_coordinate2.subtract_vector(self.vector2), CartesianCoordinate(0, 0))

    def test_to_hex(self):
        self.assertEqual(self.cartesian_coordinate2.to_hex(), self.hex_coordinate1)
        self.assertEqual(self.cartesian_coordinate3.to_hex(), HexCoordinate(-6, -4))

    def test_to_index(self):
        self.assertEqual(self.cartesian_coordinate1.to_index(), self.index1)
        self.assertEqual(self.cartesian_coordinate3.to_index(), None)

    def test_from_index(self):
        self.assertEqual(CartesianCoordinate.from_index(self.index1, 8, 8), self.cartesian_coordinate1)
        self.assertRaises(IndexError, CartesianCoordinate.from_index, self.index2, 8, 8)


class TestHexCoordinate(unittest.TestCase):
    def setUp(self):
        self.hex_coordinate1 = HexCoordinate(3, 4)
        self.hex_coordinate2 = HexCoordinate(5, 12)
        self.hex_coordinate3 = HexCoordinate(-3, -4)
        self.hex_coordinate4 = HexCoordinate(1, -1)
        self.hex_coordinate5 = HexCoordinate(3, 4)
        self.vector1 = Vector(3, 4)
        self.vector2 = Vector(5, 12)
        self.vector3 = Vector(-3, -4)
        self.vector4 = Vector(1, -1)
        self.vector5 = Vector(3, 4)
        self.cartesian_coordinate1 = CartesianCoordinate(2, 4)

    def test_to_cartesian(self):
        self.assertEqual(self.hex_coordinate1.to_cartesian(), self.cartesian_coordinate1)
        self.assertEqual(self.hex_coordinate2.to_cartesian(), CartesianCoordinate(3, 12))

    def test_add_vector(self):
        self.assertEqual(self.hex_coordinate1.add_vector(self.vector1), HexCoordinate(6, 8))
        self.assertEqual(self.hex_coordinate2.add_vector(self.vector2), HexCoordinate(10, 24))

    def test_subtract_vector(self):
        self.assertEqual(self.hex_coordinate1.subtract_vector(self.vector1), HexCoordinate(0, 0))
        self.assertEqual(self.hex_coordinate2.subtract_vector(self.vector2), HexCoordinate(0, 0))

    def test_get_neighbors(self):
        self.assertEqual(self.hex_coordinate1.get_neighbors(),
                         [HexCoordinate(4, 3), HexCoordinate(1, 4), HexCoordinate(4, 5), HexCoordinate(2, 5),
                          HexCoordinate(5, 4), HexCoordinate(2, 3)])
        self.assertEqual(self.hex_coordinate2.get_neighbors(),
                         [HexCoordinate(6, 11), HexCoordinate(3, 12), HexCoordinate(6, 13), HexCoordinate(4, 13),
                          HexCoordinate(7, 12), HexCoordinate(4, 11)])


class TestMove(unittest.TestCase):
    def setUp(self):
        self.move1 = Move(team_enum=TeamEnum.ONE, from_value=HexCoordinate(3, 4), to_value=HexCoordinate(5, 12))
        self.move2 = Move(team_enum=TeamEnum.TWO, from_value=HexCoordinate(5, 12), to_value=HexCoordinate(3, 4))
        self.move3 = Move(team_enum=TeamEnum.ONE, from_value=None, to_value=HexCoordinate(5, 12))
        self.move4 = Move(team_enum=TeamEnum.TWO, from_value=None, to_value=HexCoordinate(3, 4))

    def test_get_delta(self):
        self.assertEqual(self.move1.get_delta(), 8.246211251235321)
        self.assertEqual(self.move2.get_delta(), 8.246211251235321)
        self.assertEqual(self.move3.get_delta(), 0.0)
        self.assertEqual(self.move4.get_delta(), 0.0)

    def test_reversed(self):
        self.assertEqual(self.move1.reversed(), self.move2)
        self.assertEqual(self.move2.reversed(), self.move1)
        self.assertEqual(self.move3.reversed(),
                         Move(team_enum=TeamEnum.ONE, from_value=None, to_value=HexCoordinate(5, 12)))
        self.assertEqual(self.move4.reversed(),
                         Move(team_enum=TeamEnum.TWO, from_value=None, to_value=HexCoordinate(3, 4)))


class TestPenguin(unittest.TestCase):
    def setUp(self):
        self.penguin1 = Penguin(HexCoordinate(3, 4), TeamEnum.ONE)
        self.penguin2 = Penguin(HexCoordinate(3, 4), TeamEnum.TWO)
        self.penguin3 = Penguin(HexCoordinate(5, 12), TeamEnum.ONE)
        self.penguin4 = "not a penguin"

    def test_equal(self):
        self.assertEqual(self.penguin1, self.penguin1)
        self.assertNotEqual(self.penguin1, self.penguin2)
        self.assertNotEqual(self.penguin1, self.penguin3)
        self.assertNotEqual(self.penguin1, self.penguin4)

    def test_repr(self):
        self.assertEqual(repr(self.penguin1), "Penguin(HexCoordinate(3, 4), ONE)")
        self.assertEqual(repr(self.penguin2), "Penguin(HexCoordinate(3, 4), TWO)")
        self.assertEqual(repr(self.penguin3), "Penguin(HexCoordinate(5, 12), ONE)")


class TestField(unittest.TestCase):
    def setUp(self):
        self.penguin1 = Penguin(HexCoordinate(3, 4), TeamEnum.ONE)
        self.penguin2 = Penguin(HexCoordinate(5, 12), TeamEnum.TWO)
        self.field1 = Field(HexCoordinate(3, 4), self.penguin1, 5)
        self.field2 = Field(HexCoordinate(5, 12), None, 0)
        self.field3 = Field(HexCoordinate(3, 4), self.penguin2, 5)
        self.field4 = "not a field"

    def test_is_empty(self):
        self.assertFalse(self.field1.is_empty())
        self.assertTrue(self.field2.is_empty())

    def test_is_occupied(self):
        self.assertTrue(self.field1.is_occupied())
        self.assertFalse(self.field2.is_occupied())

    def test_get_fish(self):
        self.assertEqual(self.field1.get_fish(), 5)
        self.assertEqual(self.field2.get_fish(), 0)

    def test_get_team(self):
        self.assertEqual(self.field1.get_team(), TeamEnum.ONE)
        self.assertEqual(self.field2.get_team(), None)

    def test_eq(self):
        self.assertTrue(self.field1 == self.field1)
        self.assertFalse(self.field1 == self.field2)
        self.assertFalse(self.field1 == self.field3)
        self.assertFalse(self.field1 == self.field4)


class TestTeam(unittest.TestCase):
    def setUp(self):
        self.team_one = Team(name=TeamEnum.ONE, fish=10,
                             penguins=[Penguin(coordinate=HexCoordinate(1, 1), team_enum=TeamEnum.ONE)],
                             moves=[Move(team_enum=TeamEnum.ONE,
                                         from_value=HexCoordinate(1, 1), to_value=HexCoordinate(2, 2))])

    def test_get_penguins(self):
        self.assertEqual(self.team_one.get_penguins(),
                         [Penguin(coordinate=HexCoordinate(1, 1), team_enum=TeamEnum.ONE)])

    def test_get_moves(self):
        self.assertEqual(self.team_one.get_moves(), [Move(team_enum=TeamEnum.ONE,
                                                          from_value=HexCoordinate(1, 1),
                                                          to_value=HexCoordinate(2, 2))])

    def test_color(self):
        self.assertEqual(self.team_one.color(), TeamEnum.ONE.value)

    def test_opponent(self):
        self.assertWarns(Warning, self.team_one.opponent)

    def test_eq(self):
        team = Team(name=TeamEnum.ONE, fish=10,
                    penguins=[Penguin(coordinate=HexCoordinate(1, 1), team_enum=TeamEnum.ONE)],
                    moves=[Move(team_enum=TeamEnum.ONE,
                                from_value=HexCoordinate(1, 1), to_value=HexCoordinate(2, 2))])
        self.assertTrue(self.team_one == team)


class TestBoard(unittest.TestCase):

    def setUp(self):
        self.coord1 = CartesianCoordinate(0, 0).to_hex()
        self.coord2 = CartesianCoordinate(1, 0).to_hex()
        self.coord3 = CartesianCoordinate(0, 1).to_hex()
        self.coord4 = CartesianCoordinate(1, 1).to_hex()
        self.coord5 = CartesianCoordinate(0, 2).to_hex()
        self.coord6 = CartesianCoordinate(1, 2).to_hex()
        self.coord7 = CartesianCoordinate(0, 3).to_hex()
        self.coord8 = CartesianCoordinate(1, 3).to_hex()
        self.coord9 = CartesianCoordinate(0, 4).to_hex()
        self.coord10 = CartesianCoordinate(1, 4).to_hex()
        self.penguin1 = Penguin(self.coord2, TeamEnum.ONE)
        self.penguin2 = Penguin(self.coord7, TeamEnum.TWO)
        self.field1 = Field(self.coord1, None, 0)
        self.field2 = Field(self.coord2, self.penguin1, 0)
        self.field3 = Field(self.coord3, None, 2)
        self.field4 = Field(self.coord4, None, 3)
        self.field5 = Field(self.coord5, None, 4)
        self.field6 = Field(self.coord6, None, 0)
        self.field7 = Field(self.coord7, self.penguin2, 0)
        self.field8 = Field(self.coord8, None, 2)
        self.field9 = Field(self.coord9, None, 3)
        self.field10 = Field(self.coord10, None, 4)
        self.game_field = [[self.field1, self.field2],
                           [self.field3, self.field4],
                           [self.field5, self.field6],
                           [self.field7, self.field8],
                           [self.field9, self.field10]]
        self.board = Board(self.game_field)

    def test_get_empty_fields(self):
        empty_fields = self.board.get_empty_fields()
        self.assertEqual(len(empty_fields), 2)
        self.assertIn(self.field1, empty_fields)
        self.assertIn(self.field6, empty_fields)

    def test_is_occupied(self):
        self.assertTrue(self.board.is_occupied(self.field2.coordinate))
        self.assertFalse(self.board.is_occupied(self.field3.coordinate))

    def test_width(self):
        self.assertEqual(self.board.width(), 2)

    def test_height(self):
        self.assertEqual(self.board.height(), 5)

    def test_is_valid(self):
        self.assertTrue(self.board.is_valid(self.field7.coordinate))
        self.assertTrue(self.board.is_valid(self.field8.coordinate))
        self.assertFalse(self.board.is_valid(HexCoordinate(x=8, y=0)))
        self.assertFalse(self.board.is_valid(HexCoordinate(x=0, y=-1)))

    def test__get_field(self):
        self.assertEqual(self.board._get_field(0, 0), self.field1)
        self.assertEqual(self.board._get_field(1, 0), self.field2)

    def test_get_field(self):
        # Test valid input
        position = HexCoordinate(0, 0)
        expected_field = self.field1
        result = self.board.get_field(position)
        self.assertEqual(result, expected_field)

        # Test invalid input
        position = HexCoordinate(self.board.width(), self.board.height())
        with self.assertRaises(IndexError):
            self.board.get_field(position)

    def test_get_field_or_none(self):
        field = self.board.get_field_or_none(HexCoordinate(0, 0))
        self.assertEqual(field, self.field1)
        field = self.board.get_field_or_none(HexCoordinate(100, 100))
        self.assertIsNone(field)

    def test_get_field_by_index(self):
        field = self.board.get_field_by_index(5)
        self.assertEqual(field, self.field6)
        with self.assertRaises(IndexError):
            self.board.get_field_by_index(20)

    def test_get_all_fields(self):
        all_fields = self.board.get_all_fields()
        self.assertEqual(len(all_fields), self.board.width() * self.board.height())
        for row in self.board.board:
            for field in row:
                self.assertIn(field, all_fields)

    def test_compare_to(self):
        board1 = Board(copy.deepcopy(self.game_field))
        board2 = Board(copy.deepcopy(self.game_field))
        self.assertEqual(len(board1.compare_to(board2)), 0)

        board2.board[0][0] = Field(self.coord1, None, 5)
        self.assertNotEqual(len(board1.compare_to(board2)), 0)

        board1.board[0][0].penguin = Penguin(HexCoordinate(0, 0), TeamEnum.ONE)
        self.assertNotEqual(len(board1.compare_to(board2)), 0)

    def test_contains(self):
        field = Field(CartesianCoordinate(1, 1).to_hex(), None, 3)
        self.assertTrue(self.board.contains(field))
        field = Field(CartesianCoordinate(9, 9).to_hex(), None, -1)
        self.assertFalse(self.board.contains(field))
        field = Field(CartesianCoordinate(1, 0).to_hex(), Penguin(CartesianCoordinate(1, 0).to_hex(), TeamEnum.ONE), 0)
        self.assertTrue(self.board.contains(field))

    def test_contains_all(self):
        # Test that all fields are in the board
        self.assertTrue(self.board.contains_all([self.field1, self.field2, self.field3]))

        # Test that one field is missing
        field4 = Field(HexCoordinate(4, 4), None, 0)
        self.assertFalse(self.board.contains_all([self.field1, self.field2, self.field3, field4]))

        # Test that no fields are in the board
        self.assertFalse(self.board.contains_all([]))

    def test_get_moves_in_direction(self):
        board = Board(self.game_field)
        expected_moves = [
            Move(team_enum=TeamEnum.ONE, from_value=self.field2.coordinate, to_value=self.field3.coordinate)]
        self.assertEqual(board.get_moves_in_direction(self.field2.coordinate, Vector().directions[3], TeamEnum.ONE),
                         expected_moves)

        expected_moves = [
            Move(team_enum=TeamEnum.TWO, from_value=self.field7.coordinate, to_value=self.field10.coordinate)]
        self.assertEqual(board.get_moves_in_direction(self.field7.coordinate, Vector().directions[2], TeamEnum.TWO),
                         expected_moves)

    def test_possible_moves_from(self):
        expected_moves = [
            Move(team_enum=TeamEnum.ONE, from_value=self.coord2, to_value=self.coord4),
            Move(team_enum=TeamEnum.ONE, from_value=self.coord2, to_value=self.coord3)
        ]

        self.assertEqual(self.board.possible_moves_from(self.coord2, TeamEnum.ONE), expected_moves)

        with self.assertRaises(IndexError):
            self.board.possible_moves_from(HexCoordinate(-1, -5), TeamEnum.ONE)

    def test_get_penguins(self):
        penguins = self.board.get_penguins()
        self.assertEqual(len(penguins), 2)
        self.assertEqual(penguins[0], self.penguin1)
        self.assertEqual(penguins[1], self.penguin2)

    def test_get_teams_penguins(self):
        penguins_one = self.board.get_teams_penguins(TeamEnum.ONE)
        self.assertEqual(len(penguins_one), 1)
        self.assertEqual(penguins_one[0], self.penguin1)

        penguins_two = self.board.get_teams_penguins(TeamEnum.TWO)
        self.assertEqual(len(penguins_two), 1)
        self.assertEqual(penguins_two[0], self.penguin2)

    def test_get_most_fish(self):
        self.assertEqual(len(self.board.get_most_fish()), 2)
        self.assertEqual(self.board.board[2][0].fish, self.board.get_most_fish()[0].fish)
