#include <stdio.h>

int main() {
  int square_a_line, square_b_line, i, j, k, l, flag;
  scanf("%d %d", &square_a_line, &square_b_line);
  char array[square_a_line];
  int square_a[square_a_line][square_a_line], square_b[square_b_line][square_b_line];
  for(i = 0; i < square_a_line; i++) {
    scanf("%s", array);
    for(j = 0; j < square_a_line; j++) {
      square_a[i][j] = (array[j] == '#')? 0 : 1;
    }
  }
  for(i = 0; i < square_b_line; i++) {
    scanf("%s", array);
    for(j = 0; j < square_b_line; j++) {
      square_b[i][j] = (array[j] == '#')? 0 : 1;
    }
  }
  for(i = 0; i < square_a_line - square_b_line; i++) {
    for(j = 0; j < square_a_line - square_b_line; j++) {
      flag = 0;
      for(k = i; k < i + square_b_line; k++) {
        for(l = j; l < j + square_b_line; l++) {
          if(square_a[i+k][j+l] == square_b[k][l]) {
            flag++;
          } else {
            break;
          }
        }
      }
      if(flag == square_b_line * square_b_line) {
        break;
      }
    }
  }
  if(flag == square_b_line * square_b_line) {
    printf("Yes\n");
  } else {
    printf("No\n");
  }
  return 0;
}
