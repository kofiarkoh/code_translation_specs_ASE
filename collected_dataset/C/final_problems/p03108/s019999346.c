#include <stdio.h>
typedef long long ll;

int
main(int argc, char *argv[])
{
  int n, m;
  scanf("%d%d", &n, &m);
  int a[100000], b[100000];
  for(int i = 0; i < m; i++){
    scanf("%d%d", &a[i], &b[i]);
    a[i]--; b[i]--;
  }

  int r[100000]; ll t[100000];
  for(int i = 0; i < n; i++){
    r[i] = i; t[i] = 1;
  }
 

  ll c[100000];
  c[m - 1] = ((long long)n * ((long long)n - 1)) / 2; 

  
  for(int i = m - 1; i > 0; i--){
    int temp;
    temp = a[i];
    while(r[temp] != temp){
      temp = r[temp];
    }
    r[a[i]] = temp;
    temp = b[i];
    while(r[temp] != temp){
      temp = r[temp];
    }
    r[b[i]] = temp;
    if(r[a[i]] != r[b[i]]){
      c[i - 1] = c[i] - t[r[a[i]]] * t[r[b[i]]];
      t[r[a[i]]] += t[r[b[i]]];
      r[r[b[i]]] = r[a[i]];
    }else{
      c[i - 1] = c[i];
    }
  }
  for(int i = 0; i < m; i++){
    printf("%lld\n", c[i]);
  }
  return 0;
}

  
  
