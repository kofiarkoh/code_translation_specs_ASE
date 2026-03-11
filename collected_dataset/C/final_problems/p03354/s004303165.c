#include<stdio.h>
//小さい順にソート
int c[200002][2];
void merge(int i,int j,int s[][2]){
  if(i!=j){
    merge(i,(i+j)/2,s);
    merge((i+j)/2+1,j,s);
    int a=i,b=(i+j)/2+1;
    int d;
    for(d=i;d<=j;d++){
      if((i+j)/2<a){
        c[d][0]=s[b][0];
        c[d][1]=s[b][1];
        b++;
      }
      else if(j<b){
        c[d][0]=s[a][0];
        c[d][1]=s[a][1];
        a++;
      }
      else if(s[a][0]<s[b][0] || (s[a][0]==s[b][0] && s[a][1]<s[b][1])){
        c[d][0]=s[a][0];
        c[d][1]=s[a][1];
        a++;
      }
      else{
        c[d][0]=s[b][0];
        c[d][1]=s[b][1];
        b++;
      }
    }
    for(d=i;d<=j;d++){
      s[d][0]=c[d][0];
      s[d][1]=c[d][1];
    }
  }
}

int main(){
  int n,m,i;
  scanf("%d%d",&n,&m);
  int p[n+1],x[2*m+1][2],mass[n+1];
  for(i=1;i<=n;i++){
    scanf("%d",&p[i]);
    mass[i]=i;
  }
  for(i=1;i<=m;i++){
    scanf("%d%d",&x[i][0],&x[i][1]);
    x[i+m][0]=x[i][1];
    x[i+m][1]=x[i][0];
  }
  merge(1,2*m,x);
  for(i=1;i<=2*m;i++)
    if(mass[x[i][1]]>mass[x[i][0]])
      mass[x[i][1]]=mass[x[i][0]];
  int count=0;
  for(i=1;i<=n;i++)
    if(mass[i]==mass[p[i]])
      count++;
  printf("%d\n",count);
  return 0;
}
