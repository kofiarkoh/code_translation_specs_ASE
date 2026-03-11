#include <stdio.h>
#include <stdlib.h>

typedef struct dice{
  int pip[6];
  int top, front, right;
}Dice;

Dice *createDice(int *a);             /* ?????????????????? */
int DiceRight(Dice D, int t, int f);  /* ????????????????????´?????°??? */
int checkDice(Dice D1, Dice D2);      /* ?????????????????????????????? */


int main(void){
  Dice *D[100];
  int i, j, k = 0, n, a[6];

  scanf("%d", &n);
  for(i = 0; i < n; i++){
    for(j = 0; j < 6; j++){
      scanf("%d", &a[j]);
    }
    D[i] = createDice(a);
  }
  for(i = 0; i < n-1; i++){
    for(j = i+1; j < n; j++){
      if(checkDice(*D[i], *D[j]) != 0){
        k++;
        break;
      }
    }
  }
  if(k == 0){
    printf("Yes\n");
  }else{
    printf("No\n");
  }
  for(i = 0; i < n; i++){
    free(D[i]);
  }
  return 0;
}


Dice *createDice(int *a){
  Dice *p;
  int i;

  p = (Dice *)malloc(sizeof(Dice));
  if(p == NULL){
    printf("?????¢????¢??????¨??????\n");
    return NULL;
  }
  for(i = 0; i < 6; i++){
    p->pip[i] = a[i];
  }
  p->top = 0;
  p->front = 1;
  p->right = 2;
  return p;
}

int DiceRight(Dice D, int t, int f){
  static int check[6][6] = {{-1, 2, 4, 1, 3, -1},
			    {3, -1, 0, 5, -1, 2},
			    {1, 5, -1, -1, 0, 4},
			    {4, 0, -1, -1, 5, 1},
			    {2, -1, 5, 0, -1, 3},
			    {-1, 3, 1, 4, 2, -1}};
  return D.pip[check[t][f]];
}


int checkDice(Dice D1, Dice D2){
  int i, j, s = 0, c[6] = {-1,-1,-1,-1,-1,-1};

  for(i = 0; i < 3; i++){
    for(j = 0; j < 6; j++){
      if(c[j] == -1 && c[5-j] == -1){
	if(D1.pip[i] == D2.pip[j] && D1.pip[5-i] == D2.pip[5-j]){
	  s++;
	  c[j] = i;
	  c[5-j] = 5-i;
	  break;
	}
      }
    }
  }
  if(s != 3){
    return 0;
  }
  if(DiceRight(D2, c[0], c[1]) != D2.pip[c[2]]){
    return 0;
  }
  return 1;
}