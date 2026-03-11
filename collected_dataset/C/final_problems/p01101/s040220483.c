#include<stdio.h>
 
int main(void)
{
  int i,j,l,m,z[1000],a,b;
 
  while(1){
      scanf("%d%d",&l,&m);
      if(l==0&&m==0) break;
      for(i=0;i<=l-1;i++){
      scanf("%d",&z[i]);
      }
      b=0;
      for(i=0;i<=l-1;i++){
	for(j=i+1;j<=l-1;j++){
          a=z[i]+z[j];
          if(a<=m && b<a) b=a;
	}
      }
      if(b==0) printf("NONE\n");
      else printf("%d\n",b);
  }
  return 0;
}


