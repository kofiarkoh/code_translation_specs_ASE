#include<stdio.h>

#define M  1000
#define M_1 10000


int n, A[M], s;
int b[M], T[M_1 + 1];


int partition(int A[], int l, int r){
  int i, j;
  int t, x;
  x = A[r];
  i = l - 1;

  for( j = l ; j < r ; j++ ){
    if(A[j] <= x){
      i++;
      t = A[i];
      A[i] = A[j];
      A[j] = t;
    }
  }

  t = A[i + 1];
  A[i + 1] = A[r];
  A[r] = t;

  return i + 1;
}
void quickSort(int A[], int l, int r){
  int q;
  if(l < r){
    q = partition(A, l, r);
    quickSort(A, l, q - 1);
    quickSort(A, q + 1, r);
  }
}


int solve(){
  int i, cur, S, m, an, v,   ans = 0 ;

  int V[M];
  for(i = 0 ; i < n ; i++){
    b[i] = A[i];
    V[i] = 0;
  }

  quickSort(b, 0, n - 1);

  for(i = 0 ; i < n ; i++) T[b[i]] = i;
  for(i = 0 ; i < n ; i++){
    if(V[i]) continue;
    cur = i;
    S = 0;
    m = M_1;
    an = 0;
    while(1){
      V[cur] = 1;
      an++;
      v = A[cur];

      if(v < m) m = v;

      S += v;
      cur = T[v];
      if(V[cur]) break;
    }
    if(S + (an -2) * m < m + S + (an + 1) * s) ans += S + (an -2) * m;
    else ans +=  m + S + (an + 1) * s;
  }
  return ans;
}



int main(){
  int i;

  scanf("%d", &n);
  s = M_1;
  for(i = 0 ; i < n ; i++){
    scanf("%d", &A[i]);
    if(A[i] < s) s = A[i];
  }

  int ans = solve();
  printf("%d\n", ans);

  return 0;
}

