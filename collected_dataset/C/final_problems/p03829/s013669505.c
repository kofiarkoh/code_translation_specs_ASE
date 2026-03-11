#include <stdio.h>

int main(){
  int i, N, tele, walk;
  long long int walk_sum = 0, tele_sum = 0;//初期値
  double walk_max;
  int x[100000];//街の座標のリスト

  scanf("%d %d %d\n", &N, &walk, &tele);
  walk_max = (double)tele/walk;

  for(i = 0; i < N ; i++){
    scanf("%d", &x[i]);
  }



  for(i = 0; i < N-1; i++){
    if( x[i+1] - x[i] > walk_max){//テレポート優先
      tele_sum += tele;
    }else{//歩行優先
      walk_sum += ( x[i+1] - x[i] ) * walk;
    }
  }

  printf("%lld\n", tele_sum + walk_sum);

  return 0;
}
