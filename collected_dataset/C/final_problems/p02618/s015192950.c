#include<stdio.h>
int main(){
  int i,j,k;
  int d;	//365
  int c[26];
  int score[9490];
  int t[365] = {0};
  int day1[26] = {0};
  double x = 0.36787944117;
  int tmp1=-999999,tmp2=-999999;
  int tmpi = 0;
  
  scanf("%d",&d);
  for(i=0;i<26;i++){
      scanf("%d ",&c[i]);
  }
  for(i=0;i<d*26;i++){
      scanf("%d ",&score[i]);
  }
  //st
  
  for(i=0;i<26;i++){
    if(c[i]>tmp1){
     tmp1 = c[i];
    }
    tmpi = i;
  }
  tmp1 = -999999;/*
  for(i=0;i<d;i++){
    for(j=0;j<26;j++){
      if(score[i*26+j]-c[j]*(1+day1[j])>tmp1){
       	t[i] = j;
      }
    }
  }
  */
  //en
  int p1[26]={0};
    int p2 = 0;
  for(i=0;i<d;i++){
    if(i%26==0){
     for(j=0;j<26;j++){
      p1[j]=0; 
     }
    }
    int tp = -9999999;
    int tp2 = 0;
    int xx;
    for(xx=0;xx<26;xx++){
     if(p1[xx]==0){
      tp2 = xx;
       xx=999;
     }
    }
    
    
    for(j=0;j<26;j++){
      for(k=0;k<26;k++){
     if(p1[j]==0&&score[i*26+k]>tp){
       tp = score[i*26+k];
       tp2 = k;
     }
    }}
    
    
    //p2 = (tmpi+i+tp2)%26+1;
    p2 =tp2+1;
    
    
    printf("%d\n",p2);
    
    
    
  }
 return 0; 
}











/*
#include<stdio.h>
int main(){
  int i,j,k;
  int d;	//365
  int c[26];
  int score[9490];
  int t[365] = {0};
  int day1[26] = {0};
  double x = 0.36787944117;
  int tmp1=-999999,tmp2=-999999;
  
  scanf("%d",&d);
  for(i=0;i<26;i++){
      scanf("%d ",&c[i]);
  }
  for(i=0;i<d*26;i++){
      scanf("%d ",&score[i]);
  }
  //st
  
  for(i=0;i<d;i++){
    for(j=0;j<26;j++){
      if(score[i*26+j]-c[j]*(1+day1[j])>tmp1){
       	tmp1 =  score[i*26+j]-c[j]*(1+day1[j]);
        t[i] = j;
      }
      for(k=0;k<26;k++){
        day1[k] += 1; 
      }
      day1[t[j]] = 0;
    }
  }
  
  //en
  for(i=0;i<d;i++){
      printf("%d\n",t[i]+1);
  }
 return 0; 
}
*/


