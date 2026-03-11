#include <stdio.h>

#define SIZE 300005

typedef struct stack{char elm[SIZE]; int top;}stack_t;  //スタック

//スタック操作関数
//初期化
void Reset(stack_t *s){s->top = -1;}

//プッシュ
void push(stack_t *s, char e){
  s->top++;
  s->elm[s->top] = e;
}

//ポップ
char pop(stack_t *s){
  char e;

  e = s->elm[s->top];
  s->top--;
  return e;
}

//空の判断
int isEmpty(stack_t *s){return s->top == -1;}

//位置qの括弧をひっくり返す
void Flip(char st[], int q){
  switch (st[q-1]) {
    case '(':
      st[q-1] = ')';
      break;
    case ')':
      st[q-1] = '(';
      break;
  }
}

//括弧の対応がとれているなら1,取れていないなら0を返す
int Balance(char st[]){
  stack_t s;

  Reset(&s);
  for(int i = 0; st[i] != '\0'; ++i){
    switch (st[i]) {
      case '(':
        push(&s, st[i]);
        break;
      case ')':
        if(isEmpty(&s))
          return 0;
        pop(&s);
        break;
    }
  }
  if(isEmpty(&s))
    return 1;
  else
    return 0;
}

//ひっくり返したとき括弧の対応がとれている最も左の位置を返す
int Position(char st[]){
  for(int i = 1; st[i-1] != '\0'; ++i){
    Flip(st, i);
    if(Balance(st)) //対応がとれているなら、その位置を返す
      return i;
    else  //とれていないなら、もとに戻す
      Flip(st, i);
  }
}
/*
void Print(char st[]){
  for(int j = 0; st[j] != '\0'; j++){
    printf("%c", st[j]);
  }
  printf("\n");
}
*/
int main(){
  char st[SIZE];
  int p, q, n;

  scanf("%d%d", &n, &q);
  scanf("%s", st);
  for(int i = 1; i <= q; i++){
    scanf("%d", &p);
    Flip(st, p);
    //Print(st);
    //printf("Balance %d\n", Balance(st));
    //Print(st);
    printf("%d\n", Position(st));
    //Print(st);
    //printf("Balance %d\n", Balance(st));
  }

  return 0;
}

