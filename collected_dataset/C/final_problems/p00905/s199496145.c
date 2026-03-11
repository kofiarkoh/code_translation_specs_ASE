#include <stdio.h>

int p_b[10][3], q_b[10][3];  //括弧の配列
int indent_p[10], indent_q[10];
int n;

typedef struct bracket{ int r, c, s; }bra_t;
typedef struct bracket_st{ bra_t b[8000]; }bst_t;

void  Check_b(char st[10][90], int in, int pq[10][3]) {
  int numr = 0, numc = 0, nums = 0, piriod;
  int i, j;

  for(i = 0; i < in; ++i){
    piriod = 0;
    //printf("%d %d %d\n", numr, numc, nums);
    pq[i][0] = numr; pq[i][1] = numc; pq[i][2] = nums;
    for(j = 0; st[i][j] != '\0'; ++j){
      switch (st[i][j]) {
        case '(':
          numr++;
          break;
        case '{':
          numc++;
          break;
        case '[':
          nums++;
          break;
        case ')':
          numr--;
          break;
        case '}':
          numc--;
          break;
        case ']':
          nums--;
          break;
        case '.':
          if(!j){
            for(; st[i][j] == '.'; ++j)
              piriod++;
            --j;
          }
          break;
      }
    }
    //printf("%d\n", piriod);
    indent_p[i] = piriod++;
  }
}

bst_t Indent_check(int pq[10][3], int indent[10], bst_t *bst, int in){
  bst_t oks;
  int res, f, k = 0;

  for(int i = 0; i < 8000; ++i){
    f = 1;
    for(int j = 0; j < in; ++j){
      res = (pq[j][0] * bst->b[i].r) + (pq[j][1] * bst->b[i].c) + (pq[j][2] * bst->b[i].s);
      if(res != indent[j]){
        f = 0;
        break;
      }
    }
    if(f){
      oks.b[k] = bst->b[i];
      n++;
      k++;
    }
  }
  //for(int i = 0; i < n; i++)
    //printf("%d %d %d\n", oks.b[i].r, oks.b[i].c, oks.b[i].s);
  return oks;
}

void Indent(int q[10][3], bst_t *oks, int in){
  int temp, res, f;

  for(int i = 0; i < in; i++){
    f = 1;
    for(int j = 0; j < n; j++){
      if(j != 0)
        temp = res;
      res = q[i][0] * oks->b[j].r + q[i][1] * oks->b[j].c + q[i][2] * oks->b[j].s;
      if(n == 1)
        indent_q[i] = res;
      else if(j != 0 && temp != res){
        indent_q[i] = -1;
        f = 0;
        break;
      }
    }
    if(f)
      indent_q[i] = res;
  }
}

void Allset(bst_t *bst){
  bra_t bra;
  int l = 0;

  for(int i = 1; i <= 20; ++i){
    for(int j = 1; j <= 20; ++j){
      for(int k = 1; k <= 20; ++k){
        bra.r = i; bra.c = j; bra.s = k;
        bst->b[l] = bra;
        l++;
      }
    }
  }
}

int main (){
  char p[10][90], q[10][90], st[90];  //入力文字列
  bst_t all, oks;
  int ans[10];
  int i, j;
  int pin, qin;  //行数

  Allset(&all);
  while (1) {
    scanf("%d %d", &pin, &qin);
    if(pin == 0 && qin == 0)
      return 0;
    n = 0;
    for( i = 0; i < pin; i++){
      scanf("%s", st);
      for( j = 0; st[j] != '\0'; j++)
        p[i][j] = st[j];
      p[i][j] = '\0';
    }
    //printf("%d %d %d\n", all.b[1].r, all.b[1].c, all.b[1].s);
    Check_b(p, pin, p_b);
   /*for(int i = 0; i < pin; i++){
      for(int j = 0; j < 3; j++)
        printf("%d ", p_b[i][j]);
      printf("\n");
    }
    printf("\n"); */
    oks = Indent_check(p_b, indent_p, &all, pin);
    for( i = 0; i < qin; i++){
      scanf("%s", st);
      for( j = 0; st[j] != '\0'; j++)
        q[i][j] = st[j];
      q[i][j] = '\0';
    }
    Check_b(q, qin, q_b);
  /* for(int i = 0; i < qin; i++){
      for(int j = 0; j < 3; j++)
        printf("%d ", q_b[i][j]);
      printf("\n");
    }
    printf("\n"); */
    Indent(q_b, &oks, qin);
    for( i = 0; i < qin; i++)
      printf("%d ", indent_q[i]);
    printf("\n");
  }
}

