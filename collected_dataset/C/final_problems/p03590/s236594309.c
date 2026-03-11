#include <stdio.h>

long int expon2(int n);

void seq_reflesh(long int K, unsigned short int *seq);

int main(void){
  long int N, K, i, score, mscore;//scoreがオーバー風呂しないかやや心配
  unsigned short int seq[30];
  int j, bits, m, first;

  scanf("%ld %ld", &N, &K);

  long int list[N][2];

  for(i = 0; i < N; i++){
    scanf("%ld %ld", &list[i][0], &list[i][1]);
  }

  //A_iは最大30ビット

  //立ってるビットの数　＋　1だけ候補がある（最大31
  

  first = 1;
  do{
    seq_reflesh(K, seq);
    score=0;
    //これをKとしてまずがんばる
    for(i=0; i<N;i++){
      if((K | list[i][0]) == K) score = score + list[i][1];
    }
    if(first || mscore < score){
      mscore = score;
    }
    //ここからKの下を取り崩す処理
    m = 0;
    while(seq[m]){
      m++;
    }//mは初めて0になる桁でとまる

    if(expon2(m) > K) break;
    else{
      while(!seq[m]) m++;//mが再び1になったら止まる
    }

    K = K - expon2(m);

    K = K | (expon2(m) - 1);

    first = 0;
  } while(1);

  printf("%ld\n", mscore);
  return 0;
}

long int expon2(int n){
  long int result=1;
  int i;
  for(i=0;i<n;i++) result = result * 2;
  return result;
}

void seq_reflesh(long int K, unsigned short int *seq){
  long int exp;
  int j;
  
  exp = expon2(30);
  //exp=2^30

  
  for(j=29;j>=0;j--){
    exp = exp / 2;
    if(exp & K) seq[j] = 1; //2^j and Kが0じゃなければ
    else seq[j] = 0;
  }
}
