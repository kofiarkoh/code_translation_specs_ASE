#include<stdio.h>
#include<math.h>
#define EPS 1e-10

int main(void){

  int i;
  int l[10],v[2];
  double len[10];
  double time;

  while(1){
    if(scanf("%d,",&l[0])==EOF)break;
    len[0] = l[0];
    for(i=1;i<10;i++){
      scanf("%d,",&l[i]);
      len[i] = len[i-1]+l[i];
    }
    scanf("%d,",&v[0]);
    scanf("%d ",&v[1]);

    time = len[9]/(v[0] + v[1]);

    for(i=0;i<10;i++){
      if( time*v[0] - len[i] < EPS){
	printf("%d\n",i+1);
	break;
      }
    }

  }

  return 0;
}