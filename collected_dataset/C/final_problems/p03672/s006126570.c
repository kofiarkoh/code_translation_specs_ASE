#include<stdio.h>
int main (void) {
int i,j,a,n,ans;
  char s[200];
  scanf("%s",s);
  i=0,j=0,ans;
  while(s[n]!='\0'){n++;}
  
  for(i=2;i<n;i+=2){a=0;
    for(j=0;j<(n-i)/2;j++){
      if(s[j]!=s[(n-i)/2+j]){a++;}  
    }
    if(a==0){ans=n-i;break;}
  }
  printf("%d\n",ans);
  return 0;

}
