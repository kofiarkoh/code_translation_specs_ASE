#include<stdio.h>
int main(){
  int n,m,a,b[30]={0};
  int c,i,j;
  char s[100010];
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++){
    scanf("%s",s);
    b[s[0]-'A']++;
  }
  for(c=0;1;c++){
    for(i=0;i<26;i++){
      a=b[i];
      for(j=i;j&&b[j-1]<b[j];j--)b[j]=b[j-1];
      b[j]=a;
    }
    //for(i=0;i<26;i++)printf("%d ",b[i]);printf("\n");
    for(i=0;b[i];i++);
    if(i<m)break;
    for(i=0;i<m;i++)b[i]--;
  }
  printf("%d\n",c);
  return 0;
}
