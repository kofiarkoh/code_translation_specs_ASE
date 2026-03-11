#include <stdio.h>

#define MAXSIZE 80

int main() {
  int i, j = 0, p, q, numr = 0, numc = 0, nums = 0, indent_r = -1, indent_c = -1, indent_s = -1, indentline[10], piriod = 0, count = 0,
      ro = 0, co = 0, so = 0, indent_rc = -1, indent_rs = -1, indent_cs = -1, indent_rcs = -1, rer_rcs = 0, rec_rcs = 0, res_rcs = 0,
      rer_rc = 0, rer_rs = 0, rec_rc = 0, rec_cs = 0, res_rs = 0, res_cs = 0;
  char str[MAXSIZE];

  while(1){
    numr = 0; numc = 0; nums = 0; indent_r = -1; indent_c = -1; indent_s = -1; indentline[9]; piriod = 0; count = 0;
    ro = 0; co = 0; so = 0; indent_rc = -1; indent_rs = -1; indent_cs = -1; indent_rcs = -1; rer_rcs = 0; rec_rcs = 0; res_rcs = 0;
    rer_rc = 0; rer_rs = 0; rec_rc = 0; rec_cs = 0; res_rs = 0; res_cs = 0; j = 0;
    scanf("%d %d", &p, &q);  //p,qの入力
    if(p == 0 && q == 0)
      return 0;
    while(1){
      if(count == p)
        break;
      piriod = 0;
      scanf("%s", str);  //p行分の入力
      for(i = 0; str[i] != '\0'; i++){
        switch (str[i]) {
          case '(':
            //fr = 1;
            numr++;
            break;
          case '{':
            //fc = 1;
            numc++;
            break;
          case '[':
            //fs = 1;
            nums++;
            break;
          case ')':
            numr--;
            break;
          case '}':
            numc--;
            break;
          case ']':
            nums--;
            break;
          case '.':
            if(!i){
              for(; str[i] == '.'; i++)
                piriod++;
              i--;
              if(numr > 0 && indent_r == -1){
                if(numc == 0 && nums == 0){
                  indent_r = piriod / numr;
                }
                else if(numc > 0 && nums > 0 && indent_c != -1 && indent_s != -1)
                  indent_r = (piriod - (indent_c * numc + indent_s * nums)) / numr;
                else if(numc > 0 && indent_c != -1)
                  indent_r = (piriod - indent_c * numc) / numr;
                else if(nums > 0 && indent_s != -1)
                  indent_r = (piriod - indent_s * nums) / numr;
                else{}
              }
              if(numc > 0 && indent_c == -1){
                if(numr == 0 && nums == 0){
                  indent_c = piriod / numc;
                }
                else if(numr > 0 && nums > 0 && indent_r != -1 && indent_s != -1)
                  indent_c = (piriod - (indent_r * numr + indent_s * nums)) / numc;
                else if(numr > 0 && indent_r != -1)
                  indent_c = (piriod - indent_r * numr) / numc;
                else if(nums > 0 && indent_s != -1)
                  indent_c = (piriod - indent_s * nums) / numc;
                else{}
              }
              if (nums > 0 && indent_s == -1) {
                if(numr == 0 && numc == 0){
                  indent_s = piriod / nums;
                }
                else if(numr > 0 && numc> 0 && indent_r != -1 && indent_c != -1)
                  indent_s = (piriod - (indent_r * numr + indent_c * numc)) / nums;
                else if(numr > 0 && indent_r != -1)
                  indent_s = (piriod - indent_r * numr) / nums;
                else if(numc > 0 && indent_c != -1)
                  indent_s = (piriod - indent_c * numc) / nums;
                else{}
              }
              if(numr > 0 && numc > 0 && nums > 0 && indent_r == -1 && indent_c == -1 && indent_s == -1){
                if(piriod == numr + numc + nums){
                  indent_r = 1; indent_c = 1; indent_s = 1;
                }
                else if(piriod % 60 == 0){
                  indent_r = 20; indent_c = 20; indent_s = 20;
                }
                else{
                  indent_rcs = piriod; rer_rcs = numr; rec_rcs = numc; res_rcs = nums;
                }
              }
              else if(numr > 0 && numc > 0 && indent_r == -1 && indent_c == -1){
                if(piriod == numr + numc){
                  indent_r = 1; indent_c = 1;
                }
                else if(piriod % 40 == 0){
                  indent_r = 20; indent_c = 20;
                }
                else{
                  indent_rc = piriod; rer_rc = numr; rec_rc = numc;
                }
              }
              else if(numr > 0 && nums > 0 && indent_r == -1 && indent_s == -1){
                if(piriod == numr + nums){
                  indent_r = 1; indent_c = 1; indent_s = 1;
                }
                else if(piriod % 40 == 0){
                  indent_r = 20; indent_s = 20;
                }
                else{
                  indent_rs = piriod; rer_rs = numr; res_rs = nums;
                }
              }
              else if(numc > 0 && nums > 0 && indent_c == -1 && indent_s == -1){
                if(piriod == numc + nums){
                  indent_c = 1; indent_s = 1;
                }
                if(piriod % 40 == 0){
                  indent_s = 20; indent_c = 20;
                }
                else{
                  indent_cs = piriod; rec_cs = numc; res_cs = nums;
                }
              }
            }
            break;
        }
      }
      //printf("%d\n", piriod);
      //printf("%d %d %d\n", numr, numc, nums);
      //printf("%d %d %d\n", indent_r, indent_c, indent_s);
      count++;
    }
    //printf("%d %d %d\n", indent_r, indent_c, indent_s);
    count = 0;
    while(1){
      if(count == q)
        break;
      scanf("%s", str);  //q行分の入力
      if((indent_r == -1 && ro > 0) || (indent_c == -1 && co > 0) || (indent_s == -1 && so > 0)){
        if((indent_r == -1 && ro > 0) && (indent_c == -1 && co > 0) && (indent_s == -1 && so > 0)
           && rer_rcs && rec_rcs && res_rcs
           && (rer_rcs % ro == 0 || ro % rer_rcs == 0) && (rec_rcs % co == 0 || co % rec_rcs == 0)
           && (res_rcs % so == 0 || so % res_rcs == 0)){
          if(ro > rer_rcs)
            indentline[j] = indent_rcs * (ro / rer_rcs);
          else
            indentline[j] = indent_rcs / (rer_rcs / ro);
        }
        else if((indent_r == -1 && ro > 0) && (indent_c == -1 && co > 0)
                && rer_rc && rec_rc
                && (rer_rc % ro == 0 || ro % rer_rc == 0) && (rec_rc % co == 0 || co % rec_rc == 0)){
          if(ro > rer_rc)
            indentline[j] = indent_rc * (ro / rer_rc);
          else
            indentline[j] = indent_rc / (rer_rc / ro);
        }
        else if((indent_r == -1 && ro > 0) && (indent_s == -1 && so > 0)
                && rer_rs && res_rs
                && (rer_rs % ro == 0 || ro % rer_rs == 0) && (res_rs % so == 0 || so % res_rs == 0)){
          if(ro > rer_rs)
            indentline[j] = indent_rs * (ro / rer_rs);
          else
            indentline[j] = indent_rs / (rer_rs / ro);
        }
        else if((indent_c == -1 && co > 0) && (indent_s == -1 && so > 0)
                && rec_cs && res_cs
                && (rec_cs % co == 0 || co % rec_cs == 0) && (res_cs % so == 0 || so % res_cs == 0)){
          if(co > rec_cs)
            indentline[j] = indent_cs * (co / rec_cs);
          else
            indentline[j] = indent_cs / (rec_cs / co);
        }
        else
          indentline[j] = -1;
      }
      else
        indentline[j] = indent_r * ro + indent_c * co + indent_s * so;
      for(i = 0; str[i]; i++){
        switch (str[i]) {
          case '(':
            ro++;
            break;
          case '{':
            co++;
            break;
          case '[':
            so++;
            break;
          case ')':
            ro--;
            break;
          case '}':
            co--;
            break;
          case ']':
            so--;
            break;
        }
      }
      count++;
      j++;
    }
    for(j = 0; j < q; j++)
      printf("%d ", indentline[j]);
    printf("\n");
  }
}
