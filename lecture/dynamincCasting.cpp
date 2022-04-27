#include <iostream>
using namespace std;
class Language
{
public:
    Language() { cout << "Language()\n"; };
    virtual ~Language() { cout << "~Language()\n"; };

    void Show()
    {
        cout << "This is Language Class\n";
    }
};

class Cpp : public Language
{
public:
    Cpp() { cout << "Cpp()\n"; };
    virtual ~Cpp() { cout << "~Cpp()\n"; };

    void Show()
    {
        cout << "This is Cpp Class\n";
    }
};

int main(void)
{
    Language *pLanguage = new Language();
    pLanguage->Show();

    // Down Casting
    //  자식쪽에서만 추가된 함수를 호출 하고 싶을 때
    //  자식 포인터 타입으로 일시적으로 캐스팅하여 호출
    //  dynamic cast로 확인 가능

    Cpp *pCpp = dynamic_cast<Cpp *>(pLanguage);
    if (pCpp == nullptr)
    {
        // Cpp 클래스의 포인터가 nullptr이 나올 때
        cout << "Runtime Error\n";
    }
    return 0;
}
