    #include <stdio.h>
    #include <math.h>
     
    int main(){
              
      int n;
      char s[128];
      char t[128];
      int i;
      char st[256];
      
      scanf("%d",&n);
      
      for(i=0; i<n; i++){
      	scanf("%s %s",s,t);
      }
      
      for(i=0; i<n; i++){
      	st[i*2] = s[i];
        st[(2*i)+1] = t[i];
      }
      st[2*n] = '\0';
   
      printf("%s",st);
      return 0;
     
    }