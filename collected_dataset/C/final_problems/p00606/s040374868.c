#include<stdio.h>

int main(void){
  long double p[16][9];
  int souko,goal,robo;
  char s,t,b;
  int n;
  int i,j;
  while(scanf("%d%*c",&n) && n){
    scanf("%c %c %c%*c",&s,&t,&b);
    robo=s-'A';
    goal=t-'A';
    souko=b-'A';
    for(i=0;i<16;i++) for(j=0;j<9;j++) p[i][j]=0;
    p[0][robo]=1.0;
    for(i=1;i<=n;i++){
      for(j=0;j<9;j++){
	p[i][j]=0;
	if(j==souko) continue;
	p[i][j]+=((j<3 || j-3==souko)?p[i-1][j]:p[i-1][j-3])/4.0;
	p[i][j]+=((j%3==0 || j-1==souko)?p[i-1][j]:p[i-1][j-1])/4.0;
	p[i][j]+=((j%3==2 || j+1==souko)?p[i-1][j]:p[i-1][j+1])/4.0;
	p[i][j]+=((j>5 || j+3==souko)?p[i-1][j]:p[i-1][j+3])/4.0;
      }
    }
    printf("%.8Lf\n",p[n][goal]);
  }
  return 0;
}
	   