#include<stdio.h>
int main(){
  int n,m,id,i,j,l;
  char c;
  while(scanf("%d %d",&n,&m),n||m){
    int w[110]={0};
    w[n]=-1;
    while(m--){
      scanf(" %c",&c);
      if(c=='s'){
	scanf("%d %d",&id,&l);
	id++;
	for(i=0;i<n;i++){
	  if(w[i])continue;
	  for(j=0;w[i+j]==0;j++);
	  if(j<l){
	    i+=j;
	    continue;
	  }
	  printf("%d\n",i);
	  for(j=0;j<l;j++)w[i+j]=id;
	  break;
	}
	if(i>=n)printf("impossible\n");
      }
      else{
	scanf("%d",&id);
	id++;
	for(i=0;w[i]-id;i++);
	for(;w[i]==id;i++)w[i]=0;
      }
      //for(i=0;i<n;i++)printf("%d",w[i]);printf("\n");
    }
    printf("END\n");
  }
  return 0;
}