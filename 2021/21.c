#include <stdio.h>

static int lastRoll=0;

int deterministic_dice(int l);
int roll_dice();

typedef struct {
  int position;
  int points;
} player;

typedef struct {
  player* player1;
  player* player2;
  int total_dice_rolls;
} game;

int roll_dice() {
  lastRoll = deterministic_dice(lastRoll);
  return lastRoll;
}

int deterministic_dice(int lastRoll) {
  if (lastRoll < 1 || lastRoll >= 100) return 1;
  return lastRoll + 1;
}

void movePlayer(int moves, player* p) {
  p->position = ((moves - 1) + p->position) % 10 + 1;
}


//void test_roll_dice() {
//  for (int i=0;  i < 210; ++i) {
//    roll_dice();
//    printf("i=%d roll=%d\n", i, lastRoll);
//  }
//}

void part1() {
  player p1;
  p1.position = 9; //4;
  p1.points = 0;

  player p2;
  p2.position = 10;//8;
  p2.points = 0;

  game the_game;
  the_game.player1 = &p1;
  the_game.player2 = &p2;
  the_game.total_dice_rolls = 0;

  while (1) {
    int p1_moves = roll_dice() + roll_dice() + roll_dice();
    movePlayer(p1_moves, the_game.player1);
    p1.points += p1.position;
    the_game.total_dice_rolls += 3;    
    printf("Player 1 moves to space %d for a total score of %d\n", p1.position, p1.points);    

    if (p1.points >= 1000) {
      printf("Player 1 WINS!\n");
      printf("Total Rolls: %d\n", the_game.total_dice_rolls);
      break;
    }
    
    int p2_moves = roll_dice() + roll_dice() + roll_dice();
    movePlayer(p2_moves, the_game.player2);
    p2.points += p2.position;

    the_game.total_dice_rolls += 3;

    printf("Player 2 moves to space %d for a total score of %d\n", p2.position, p2.points);

    if (p2.points >= 1000) {
      printf("Player 2 WINS!\n");
      printf("Total Rolls: %d\n", the_game.total_dice_rolls);
      break;
    }
    printf("\n");
  }

  player loser = (p1.points < p2.points) ? p1 : p2;
  printf("%d * %d = %d\n", loser.points, the_game.total_dice_rolls, loser.points * the_game.total_dice_rolls);
}

int main() {
  part1(); // 766 * 924 = 707784  
  return 0;
}
