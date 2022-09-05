import unittest

from socha.api.plugin.penguins import *


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
        c = Coordinate(5, -5)
        self.assertEqual(c.x, 5)
        self.assertEqual(c.y, -5)

    def testGetDistance(self):
        c1 = Coordinate(5, 15)
        c2 = Coordinate(15, 5)
        self.assertEqual(c1.get_distance(c2), 14.142135623730951)

    def testGetArray(self):
        c = Coordinate(15, 7)
        self.assertEqual(c.get_array().x, Coordinate(7, 7, False).x)
        self.assertEqual(c.get_array().y, Coordinate(7, 7, False).y)

    def testGetDoubleHex(self):
        c = Coordinate(7, 7, False)
        self.assertEqual(c.get_double_hex().x, Coordinate(15, 7, True).x)
        self.assertEqual(c.get_double_hex().y, Coordinate(15, 7, True).y)


class MoveTest(unittest.TestCase):
    def testMoveInit(self):
        m = Move(from_value=Coordinate(0, 0), to_value=Coordinate(15, 7))
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
        f = Field(coordinate=Coordinate(0, 0), field="ONE")
        self.assertEqual(f.coordinate.x, 0)
        self.assertEqual(f.coordinate.y, 0)
        self.assertEqual(f.field.color(), "ONE")

    def testGetEmptyField(self):
        l = []
        for i in range(7):
            l.append([])
            for j in range(7):
                l[i].append(Field(coordinate=Coordinate(j, i), field=0))
        b = Board(l)
        e = b.get_empty_fields()
        self.assertEqual(len(e), 49)


class GameStateTest(unittest.TestCase):
    b = Board(game_field=[[Field(coordinate=Coordinate(j, i), field=1) for i in range(8)] for j in range(8)])
    g = GameState(board=b, turn=0, start_team=Team(color="ONE"), fishes=Fishes(fishes_one=0, fishes_two=5),
                  last_move=Move(from_value=Coordinate(0, 0), to_value=Coordinate(7, 7)))

    def testGameStateInit(self):
        self.assertEqual(self.g.turn, 0)
        self.assertEqual(self.g.start_team.color(), Team(color="ONE").color())
        self.assertEqual(self.g.fishes.fishes_one, Fishes(fishes_one=0, fishes_two=5).fishes_one)
        self.assertEqual(self.g.fishes.fishes_two, Fishes(fishes_one=0, fishes_two=5).fishes_two)

    def test_perform_low_index_move(self):
        new_state = self.g.perform_move(Move(to_value=Coordinate(1, 1)))
        self.assertEqual(new_state.turn, 1)
        self.assertEqual(new_state.fishes.fishes_one, 1)
        self.assertEqual(new_state.fishes.fishes_two, 0)
        self.assertEqual(new_state.last_move.to_value.x, 1)
        self.assertEqual(new_state.last_move.to_value.y, 1)

    def test_perform_high_index_move(self):
        new_state = self.g.perform_move(Move(to_value=Coordinate(15, 7)))
        self.assertEqual(new_state.turn, 1)
        self.assertEqual(new_state.fishes.fishes_one, 1)
        self.assertEqual(new_state.fishes.fishes_two, 0)
        self.assertEqual(new_state.last_move.to_value.x, 15)
        self.assertEqual(new_state.last_move.to_value.y, 7)

    def test_perform_error_move(self):
        self.g.board._game_field[0][0].field = 0
        self.g.possible_moves = self.g._get_possible_moves(self.g.current_team)
        with self.assertRaises(Exception):
            self.g.perform_move(Move(to_value=Coordinate(0, 0)))
