#include<stdio.h>
#include<string.h>

static const int N = 1000;
int num;

int max(int a,int b){
  if(a>b) return a;
  return b;
}

int lcs(char *X,char *Y){
  int c[N+1][N+1];
  int m = strlen(X);
  int n = strlen(Y);
  int maxl=0;
  int i,j;
  for(i=num-1;i>=0;i--){
    X[i+1]=X[i];
    Y[i+1]=Y[i];
  }
  X[0]=' ';
  Y[0]=' ';
  for(i=1;i<=m;i++)c[i][0]=0;
  for(j=1;j<=n;j++)c[0][j]=0;

  for(i=1;i<=m;i++){
    for(j=1;j<=n;j++){
      if(X[i]==Y[j]){
	c[i][j]=c[i-1][j-1]+1;
      }else{
	c[i][j]=max(c[i-1][j],c[i][j-1]);
      }
      maxl = max(maxl,c[i][j]);
    }
  }
  
  return maxl;
}
    

int main(){
  char s1[N],s2[N];
  int i;
  scanf("%d",&num);
  for(i=0;i<num;i++){
    scanf("%s%s",s1,s2);
    printf("%d\n",lcs(s1,s2));
  }
  return 0;
}