#include <stdio.h>
#include <string.h>
int main() {
  int l,x[5]={1,1,1000000,1000000,1},i,ans=0;
  char s[100001];
  scanf("%s",s);
  l=strlen(s);
  for (i=0;i<l-1&&s[i]==s[i+1];i++) {
    x[0]++;
  }
  for (i=l-1;i>0&&s[i]==s[i-1];i--) {
    x[1]++;
  }
  for (i=x[0]-1;i<l-x[1];i++) {
    if (s[i]==s[i+1]) {
      x[4]++;
    } else if (s[i]=='0') {
      if (x[4]<x[2]) {
        x[2]=x[4];
        x[4]=1;
      }
    } else {
      if (x[4]<x[3]) {
        x[3]=x[4];
        x[4]=1;
      }
    }
  }
  x[3]++;
  x[2]++;
  for (i=0;i<5;i++) {
    if (ans<x[i]&&x[i]!=1000001) {
      ans=x[i];
    }
  }
  printf("%d\n",ans);
  return 0;
}