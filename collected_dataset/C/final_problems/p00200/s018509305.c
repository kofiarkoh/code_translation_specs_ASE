#include <stdio.h>
int i[100][100][2];
int main()
{
  int n,m,k,a,b,c,t;
  for(;;){
    scanf("%d%d",&n,&m);
    if(n==0 && m==0) break;
    for(a=0;a<m;a++){
      for(b=0;b<m;b++){
	if(a==b) i[a][b][0] = i[a][b][1] = 0;
	else i[a][b][0] = i[a][b][1] = 0x1fffffff;
      }
    }
    for(k=0;k<n;k++){
      scanf("%d%d%d%d",&a,&b,&c,&t);
      i[a-1][b-1][0] = c;
      i[a-1][b-1][1] = t;
      i[b-1][a-1][0] = c;
      i[b-1][a-1][1] = t;
    }
    for(a=0;a<m;a++){
      for(b=0;b<m;b++){
	for(c=0;c<m;c++){
	  if(i[a][b][0] > i[a][c][0] + i[c][b][0])
	    i[a][b][0] = i[a][c][0] + i[c][b][0];
	  if(i[a][b][1] > i[a][c][1] + i[c][b][1])
	    i[a][b][1] = i[a][c][1] + i[c][b][1];
	}
      }
    }
    scanf("%d",&k);
    for(t=0;t<k;t++){
      scanf("%d%d%d",&a,&b,&c);
      printf("%d\n",i[a-1][b-1][c]);
    }
  }
  return 0;
}