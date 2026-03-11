    #include<stdio.h>
    int main(){
    int N,M,i,j,x,c,d;
    int a[200001],b[200001];
    scanf("%d%d",&N,&M);
    x=0;
    for(i=0;i<M;i++){
    scanf("%d%d",&c,&d);
     if(c==1||d==N){
     a[x]=c;
     b[x]=d;
     x++;
     } 
   }
    for(i=0;i<x;i++){
      if(a[i]==1){
        for(j=0;j<x;j++){
          if(a[j]==b[i]&&b[j]==N){
          printf("POSSIBLE\n");
          return 0;
          }
        }
      }
    if(i==(x-1)) printf("IMPOSSIBLE\n");
    }
    return 0;
    }