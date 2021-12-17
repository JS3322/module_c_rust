#include <string>
#include <vector>

using namespace std;

int Check(int i)
{
    if (i == 6)
        return 1;
    if (i == 5)
        return 2;
    if (i == 4)
        return 3;
    if (i == 3)
        return 4;
    if (i == 2)
        return 5;
    if (i == 1)
        return 6;
}

// lottos_len은 배열 lottos의 길이입니다.
// win_nums_len은 배열 win_nums의 길이입니다.
vector<int> solution(vector<int> lottos, vector<int> win_nums)
{
    vector<int> answer(2);
    int Cnt_zero = 0;
    int Cnt_same = 0;
    for (int i = 0; i < lottos.size(); i++)
    {
        if (lottos[i] == 0)
        {
            Cnt_zero++;
        }
        else
        {
            for (int j = 0; j < win_nums.size(); j++)
            {
                if (lottos[i] == win_nums(j))
                {
                    Cnt_same++;
                    break;
                }
            }
        }
    }
    int High = Cnt_same + Cnt_zero;
    int Low = Cnt_same;
    High = Check(High);
    Low = Check(Low);
    answer = {High, Low};
    return answer;
}