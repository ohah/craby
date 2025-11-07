#pragma once

#include "cxx.h"
#include <functional>
#include <memory>
#include <mutex>
#include <unordered_map>

// Forward declaration to avoid including React Native headers
namespace facebook {
namespace jsi {
class Value;
} // namespace jsi
} // namespace facebook

namespace craby {
namespace crabytest {
namespace signals {

using Delegate = std::function<void(const std::string& signalName)>;
using DelegateWithValue = std::function<void(const std::string& signalName, const facebook::jsi::Value& data)>;
using DelegateArrayNumber = std::function<void(const std::string& signalName, rust::Vec<double> arr)>;
using DelegateArrayString = std::function<void(const std::string& signalName, rust::Vec<rust::String> arr)>;

class SignalManager {
public:
  static SignalManager& getInstance() {
    static SignalManager instance;
    return instance;
  }

  void emit(uintptr_t id, rust::Str name) const {
    std::lock_guard<std::mutex> lock(mutex_);
    auto it = delegates_.find(id);
    if (it != delegates_.end()) {
      it->second(std::string(name));
    }
  }

  // Array<number> 타입 emit - Rust에서 호출
  void emit_array_number(uintptr_t id, rust::Str name, rust::Slice<const double> arr) const {
    std::lock_guard<std::mutex> lock(mutex_);
    auto it = delegates_array_number_.find(id);
    if (it != delegates_array_number_.end()) {
      std::string nameStr(name.data(), name.size());
      rust::Vec<double> vec;
      vec.reserve(arr.size());
      for (size_t i = 0; i < arr.size(); ++i) {
        vec.push_back(arr[i]);
      }
      it->second(nameStr, vec);
    }
  }
  
  // Array<string> 타입 emit - Rust에서 호출
  void emit_array_string(uintptr_t id, rust::Str name, rust::Slice<const rust::Str> arr) const {
    std::lock_guard<std::mutex> lock(mutex_);
    auto it = delegates_array_string_.find(id);
    if (it != delegates_array_string_.end()) {
      std::string nameStr(name.data(), name.size());
      rust::Vec<rust::String> vec;
      vec.reserve(arr.size());
      for (size_t i = 0; i < arr.size(); ++i) {
        vec.push_back(rust::String(arr[i].data(), arr[i].size()));
      }
      it->second(nameStr, vec);
    }
  }

  void registerDelegate(uintptr_t id, Delegate delegate) const {
    std::lock_guard<std::mutex> lock(mutex_);
    delegates_.insert_or_assign(id, delegate);
  }

  void registerDelegateWithValue(uintptr_t id, DelegateWithValue delegate, DelegateArrayNumber delegateArrayNumber, DelegateArrayString delegateArrayString) const {
    std::lock_guard<std::mutex> lock(mutex_);
    delegates_with_value_.insert_or_assign(id, delegate);
    delegates_array_number_.insert_or_assign(id, delegateArrayNumber);
    delegates_array_string_.insert_or_assign(id, delegateArrayString);
  }

  void unregisterDelegate(uintptr_t id) const {
    std::lock_guard<std::mutex> lock(mutex_);
    delegates_.erase(id);
    delegates_with_value_.erase(id);
    delegates_array_number_.erase(id);
    delegates_array_string_.erase(id);
  }

private:
  SignalManager() = default;
  mutable std::unordered_map<uintptr_t, Delegate> delegates_;
  mutable std::unordered_map<uintptr_t, DelegateWithValue> delegates_with_value_;
  mutable std::unordered_map<uintptr_t, DelegateArrayNumber> delegates_array_number_;
  mutable std::unordered_map<uintptr_t, DelegateArrayString> delegates_array_string_;
  mutable std::mutex mutex_;
};

inline const SignalManager& getSignalManager() {
  return SignalManager::getInstance();
}

} // namespace signals
} // namespace crabytest
} // namespace craby
