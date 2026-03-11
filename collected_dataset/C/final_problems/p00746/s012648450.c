#include<stdio.h>

int main()
{
  int aa[4][2]={{-1,0},{0,-1},{1,0},{0,1}};
  int seihou[1000][2],an[1000][2],i,n,m,ten,muki;
  int minx=0,maxx=0,miny=0,maxy=0;
  while(1){
    scanf("%d",&n);
    if(n==0)break;
    seihou[0][0]=0;
    seihou[0][1]=0;
    for(i=1;i<n;i++){
      scanf("%d%d",&ten,&muki);
      seihou[i][0]=seihou[ten][0]+aa[muki][0];
      seihou[i][1]=seihou[ten][1]+aa[muki][1];
    }
    for(i=0;i<n;i++){
      if(minx>seihou[i][0])minx=seihou[i][0];
      else if(maxx<seihou[i][0])maxx=seihou[i][0];
      else if(miny>seihou[i][1])miny=seihou[i][1];
      else if(maxy<seihou[i][1])maxy=seihou[i][1];
    }
    an[m][0]=maxx-minx+1;
    an[m][1]=maxy-miny+1;
    m++;
    maxx=0;minx=0,miny=0,maxy=0;
  }
  for(i=0;i<m;i++){
    printf("%d %d\n",an[i][0],an[i][1]);
  }
  return 0;
}