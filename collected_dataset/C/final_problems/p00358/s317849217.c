#include <stdio.h>
int main(void) {
   int H,N,i,j,p[4][10002],x,y,c;
   for(i=0;i<10002;i++){
	   p[0][i]=0;p[1][i]=0;p[2][i]=0;p[3][i]=0;
   }
   scanf("%d %d",&H,&N);
   for(i=0;i<N;i++){
	   scanf("%d %d",&x,&y);
	   p[x][y]=9;
   }
  for(i=H;i<10002;i++){
	   p[0][i]=9;p[1][i]=9;p[2][i]=9;p[3][i]=9;
  }
  c=0;
  for(j=1;j<4;j++){
	  for(i=1;i<H;i++){
		  if(p[j][i]==0&&p[j-1][i-1]==0&&p[j-1][i]==0&&p[j][i-1]==0){
			  p[j][i]=1;p[j-1][i]=1;p[j][i-1]=1;p[j-1][i-1]=1;
			  c++;
		  }
	  }
  }/*
  for(i=0;i<H;i++){
	  printf("%d%d%d%d\n",p[0][i],p[1][i],p[2][i],p[3][i]);
  }*/
  printf("%d\n",c);
   return 0;
}

