#include<stdio.h>
#include<math.h>
int main(){
  int n,s[100000],a[10][10][10]={},z=0;
  char si[100000];
  scanf("%d",&n);
  scanf("%s",si);
  for(int i=0;i<n;i++) s[i]=si[i]-'0';
  for(int i=0;i<n-2;i++){
    for(int j=i+1;j<n-1;j++){
      for(int k=j+1;k<n;k++){
        a[s[i]][s[j]][s[k]]++;
      }
    }
    for(int l=0;l<10;l++){
      for(int j=0;j<10;j++){
        for(int k=0;k<10;k++){
          if(a[l][j][k]!=0) z++;
        }
      }
    }
    if(z==1000)break;
    z=0;
  }
 for(int i=0;i<10;i++){
   for(int j=0;j<10;j++){
     for(int k=0;k<10;k++){
       if(a[i][j][k]!=0) z++;
     }
   }
 }
  printf("%d",z);
  return 0;
}