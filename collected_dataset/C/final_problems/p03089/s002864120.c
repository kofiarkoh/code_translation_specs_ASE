#include <stdio.h>

int ok = 0;
int ansa[104] = {0};

void dfs(int b[], int taken[], int n, int ans[]){
  if(ok)  return;
  if(ans[0] == n){
    for(int i=1; i<=n; i++)
      ansa[i] = ans[i];
    ok = 1;
    return;
  }

  int count = 0;
  for(int i=1; i<=n; i++){
    if(!taken[i])  count++;
    if(!taken[i] && b[i] == count){
      ans[++ans[0]] = b[i];
      taken[i] = 1;
      dfs(b, taken, n, ans);
      ans[0]--;
      taken[i] = 0;
    }
  }
  return;
}




int main(){
  int n;  scanf("%d", &n);
  int b[104] = {0};
  for(int i=1; i<=n; i++)  scanf("%d", b+i);

  int ans[104] = {0}, taken[104] = {0};
  dfs(b, taken, n, ans);

  if(ok)
    for(int i=n; i>=1; i--)
      printf("%d\n", ansa[i]);
  else
    printf("-1\n");

  return 0;
}
