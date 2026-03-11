#include<stdio.h>
//小さい順にソート
int c[200002][2],e[200002];
void merge2(int i,int j,int s[][2]){
  if(i!=j){
    merge2(i,(i+j)/2,s);
    merge2((i+j)/2+1,j,s);
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

void merge(int i,int j,int *s){
  if(i!=j){
    merge(i,(i+j)/2,s);
    merge((i+j)/2+1,j,s);
    int a=i,b=(i+j)/2+1;
    int d;
    for(d=i;d<=j;d++){
      if((i+j)/2<a){
        e[d]=s[b];
        b++;
      }
      else if(j<b){
        e[d]=s[a];
        a++;
      }
      else if(s[a]<s[b]){
        e[d]=s[a];
        a++;
      }
      else{
        e[d]=s[b];
        b++;
      }
    }
    for(d=i;d<=j;d++)
      s[d]=e[d];
  }
}

int main(){
  int n,m,i;
  scanf("%d%d",&n,&m);
  int a[m+1][2];
  for(i=1;i<=m;i++){
    int temp1,temp2;
    scanf("%d%d",&temp1,&temp2);
    if(temp1<temp2){
      a[i][0]=temp1;
      a[i][1]=temp2;
    }
    else{
      a[i][0]=temp2;
      a[i][1]=temp1;
    }
  }
  int grp[n+1];
  for(i=1;i<=n;i++)
    grp[i]=i;
  merge2(1,m,a);
  for(i=1;i<=m;i++){
    grp[a[i][1]]=grp[a[i][0]];
  }
  merge(1,n,grp);
  int max=0,temp=1;
  for(i=1;i<=n+1;i++){
    if(grp[i]==grp[i-1] && i<=n)
      temp++;
    else{
      if(max<temp)
        max=temp;
      temp=1;
    }
  }
  printf("%d\n",max);
  return 0;
}
