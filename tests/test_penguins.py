import unittest
import random

from socha.api.plugin.penguins import *


def create_random_board() -> Board:
    game_field = []
    teams = ["ONE", "TWO"]
    one_penguins = 0
    two_penguins = 0
    for y in range(8):
        row = []
        for x in range(8):
            choice = random.choice([0, 1])
            if choice == 0:
                field = Field(CartesianCoordinate(x, y).to_hex(), random.randint(0, 4))
            else:
                if one_penguins <= 4 and two_penguins <= 4:
                    if one_penguins == two_penguins:
                        team = random.choice(teams)
                    elif one_penguins > two_penguins:
                        team = "TWO"
                    else:
                        team = "ONE"
                    field = Field(CartesianCoordinate(x, y).to_hex(), team)
                else:
                    field = Field(CartesianCoordinate(x, y).to_hex(), random.randint(0, 4))
            row.append(field)
        game_field.append(row)
    return Board(game_field)


class VectorTest(unittest.TestCase):
    def testVectorInit(self):
        v = Vector(5, -5)
        self.assertEqual(v.d_x, 5)
        self.assertEqual(v.d_y, -5)

    def testMagnitude(self):
        v = Vector(5, -5)
        self.assertEqual(v.magnitude(), 7.0710678118654755)

    def testDotProduct(self):
        v1 = Vector(5, -5)
        v2 = Vector(5, -5)
        self.assertEqual(v1.dot_product(v2), 50)

    def testCrossProduct(self):
        v1 = Vector(5, -5)
        v2 = Vector(5, -5)
        self.assertEqual(v1.cross_product(v2), 0)

    def testScalarProduct(self):
        v1 = Vector(5, -5)
        v2 = Vector(10, -10)
        self.assertEqual(v1.scalar_product(2).d_x, v2.d_x)
        self.assertEqual(v1.scalar_product(2).d_y, v2.d_y)

    def testArcTangent(self):
        v1 = Vector(5, -5)
        self.assertEqual(v1.get_arc_tangent(), -45.0)


class CoordinateTest(unittest.TestCase):
    def testCoordinateInit(self):
        c = HexCoordinate(5, -5)
        self.assertEqual(c.x, 5)
        self.assertEqual(c.y, -5)

    def testGetDistance(self):
        c1 = HexCoordinate(5, 15)
        c2 = HexCoordinate(15, 5)
        self.assertEqual(c1.distance(c2), 14.142135623730951)

    def testGetArray(self):
        c = HexCoordinate(15, 7)
        self.assertEqual(c.to_cartesian().x, HexCoordinate(7, 7).x)
        self.assertEqual(c.to_cartesian().y, HexCoordinate(7, 7).y)


class MoveTest(unittest.TestCase):
    def testMoveInit(self):
        m = Move(from_value=HexCoordinate(0, 0), to_value=HexCoordinate(15, 7))
        self.assertEqual(m.from_value.x, 0)
        self.assertEqual(m.from_value.y, 0)
        self.assertEqual(m.to_value.x, 15)
        self.assertEqual(m.to_value.y, 7)


class TeamTest(unittest.TestCase):
    def testTeamInit(self):
        t = Team(color="ONE")
        self.assertEqual(t.color(), "ONE")


class FieldTest(unittest.TestCase):
    def testFieldInit(self):
        f = Field(coordinate=HexCoordinate(0, 0), field="ONE")
        self.assertEqual(f.coordinate.x, 0)
        self.assertEqual(f.coordinate.y, 0)
        self.assertEqual(f.field.color(), "ONE")

    def testGetEmptyField(self):
        list_value = []
        for i in range(7):
            list_value.append([])
            for j in range(7):
                list_value[i].append(Field(coordinate=CartesianCoordinate(j, i).to_hex(), field=0))
        b = Board(list_value)
        e = b.get_empty_fields()
        self.assertEqual(len(e), 49)


class BoardTest(unittest.TestCase):
    b = create_random_board()

    def test_get_empty_fields(self):
        empty_fields = self.b.get_empty_fields()

        for field in empty_fields:
            self.assertTrue(field.is_empty())

    def test_is_occupied(self):
        for x in range(self.b.width() - 1):
            for y in range(self.b.height() - 1):
                field = self.b._get_field(x, y)
                if field.is_occupied():
                    self.assertTrue(self.b.is_occupied(field.coordinate))
                else:
                    self.assertFalse(self.b.is_occupied(field.coordinate))

    def test_is_valid(self):
        for x in range(self.b.width()):
            for y in range(self.b.height()):
                self.assertTrue(self.b.is_valid(CartesianCoordinate(x, y).to_hex()))

        self.assertFalse(self.b.is_valid(CartesianCoordinate(-1, 0).to_hex()))
        self.assertFalse(self.b.is_valid(CartesianCoordinate(0, -1).to_hex()))
        self.assertFalse(self.b.is_valid(CartesianCoordinate(8, 0).to_hex()))
        self.assertFalse(self.b.is_valid(CartesianCoordinate(0, 8).to_hex()))

    def test_get_field(self):
        valid_coordinates = []
        for y in range(self.b.height() - 1):
            for x in range(self.b.width() - 1):
                valid_coordinates.append(CartesianCoordinate(x, y).to_hex())
        random_coordinates = random.choice(valid_coordinates)
        field = self.b.get_field(random_coordinates)
        self.assertEqual(field.coordinate, random_coordinates)
        self.assertTrue(isinstance(field.field, int) or isinstance(field.field, Team))

        invalid_coordinates = [CartesianCoordinate(-1, 0).to_hex(), CartesianCoordinate(0, -1).to_hex(),
                               CartesianCoordinate(self.b.width(), 0).to_hex(),
                               CartesianCoordinate(0, self.b.height()).to_hex()]
        random_coordinates = random.choice(invalid_coordinates)
        with self.assertRaises(IndexError):
            self.b.get_field(random_coordinates)

    def test_get_field_by_index(self):
        random_indices = random.randint(0, 63)
        field = self.b.get_field_by_index(random_indices)
        self.assertEqual(field.coordinate, CartesianCoordinate(random_indices % self.b.width(),
                                                               int(random_indices / self.b.width())).to_hex())
        self.assertTrue(isinstance(field.field, int) or isinstance(field.field, Team))

        random_indices = random.randint(-100, -1)
        with self.assertRaises(IndexError):
            self.b.get_field_by_index(random_indices)

    def test_get_most_fish(self):
        most_fish_fields = self.b.get_most_fish()
        self.assertTrue(isinstance(most_fish_fields, list))
        self.assertTrue(all(isinstance(field, Field) for field in most_fish_fields))
        self.assertTrue(all(isinstance(field.field, int) for field in most_fish_fields))
        self.assertTrue(all(field.field >= 0 for field in most_fish_fields))


class GameStateTest(unittest.TestCase):
    b = Board(game_field=[[Field(coordinate=CartesianCoordinate(j, i).to_hex(), field=1) for i in range(8)] for j in
                          range(8)])
    g = GameState(board=b, turn=1, start_team=Team(color="ONE"), fishes=Fishes(fishes_one=1, fishes_two=0),
                  last_move=Move(from_value=None, to_value=HexCoordinate(7, 7)))

    def testGameStateInit(self):
        self.assertEqual(self.g.turn, 1)
        self.assertEqual(self.g.start_team.color(), Team(color="ONE").color())
        self.assertEqual(self.g.fishes.fishes_one, Fishes(fishes_one=1, fishes_two=0).fishes_one)
        self.assertEqual(self.g.fishes.fishes_two, Fishes(fishes_one=1, fishes_two=0).fishes_two)

    def test_perform_low_index_move(self):
        new_state = self.g.perform_move(Move(to_value=HexCoordinate(1, 1)))
        self.assertEqual(new_state.turn, 2)
        self.assertEqual(new_state.fishes.fishes_one, 1)
        self.assertEqual(new_state.fishes.fishes_two, 1)
        self.assertEqual(new_state.last_move.to_value.x, 1)
        self.assertEqual(new_state.last_move.to_value.y, 1)

    def test_perform_high_index_move(self):
        new_state = self.g.perform_move(Move(to_value=HexCoordinate(15, 7)))
        self.assertEqual(new_state.turn, 2)
        self.assertEqual(new_state.fishes.fishes_one, 1)
        self.assertEqual(new_state.fishes.fishes_two, 1)
        self.assertEqual(new_state.last_move.to_value.x, 15)
        self.assertEqual(new_state.last_move.to_value.y, 7)

    def test_perform_error_move(self):
        self.g.board._game_field[0][0].field = 0
        self.g.possible_moves = self.g._get_possible_moves(self.g.current_team)
        with self.assertRaises(Exception):
            self.g.perform_move(Move(to_value=HexCoordinate(0, 0)))
