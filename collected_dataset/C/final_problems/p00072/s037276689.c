#include<stdio.h>
int main(){
  int n,m,a[100],b[100],c[100],f,pr,s,t,i,j,r[100];
  while(scanf("%d",&n),n){
    scanf("%d",&m);
    for(i=0;i<n;i++){
      r[i]=i;
      scanf("%d,%d,%d",&a[i],&b[i],&c[i]);
      a[i]++;
      b[i]++;
      for(j=i;j;j--){
	if(c[r[j-1]]>c[r[j]]){
	  t=r[j-1];
	  r[j-1]=r[j];
	  r[j]=t;
	}
      }
    }
    int p[101]={0};
    int pi[101]={0};//for(i=0;i<m;i++)printf("%d ",c[r[i]]);printf("\n");
    for(i=s=pr=0;i<m;i++){
      f=0;
      if(pi[a[r[i]]])f+=1;
      if(pi[b[r[i]]])f+=2;//printf("\n%d\n",f);
      if(f!=3||p[pi[a[r[i]]]]-p[pi[b[r[i]]]])s+=c[r[i]]/100-1;
      if(f==0)pi[a[r[i]]]=pi[b[r[i]]]=p[++pr]=pr;
      if(f==1)pi[b[r[i]]]=pi[a[r[i]]];
      if(f==2)pi[a[r[i]]]=pi[b[r[i]]];
      if(f==3&&p[pi[a[r[i]]]]-p[pi[b[r[i]]]])p[pi[a[r[i]]]]=p[pi[b[r[i]]]];
      //for(j=0;j<=m;j++)printf("%d %d %d\n",j,pi[j],p[j]);
    }
    printf("%d\n",s);
  }
  return 0;
}